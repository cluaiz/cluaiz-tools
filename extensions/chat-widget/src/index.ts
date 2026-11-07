/**
 * @cluaiz/extension-chat-widget
 * Modular embeddable web chat widget extension for Cluaiz Engine
 */

import { WIDGET_STYLES } from './constants/styles';
import { ConfigService } from './services/ConfigService';
import { Launcher } from './components/Launcher';
import { ChatWindow } from './components/ChatWindow';

export { Launcher } from './components/Launcher';
export { ChatWindow } from './components/ChatWindow';
export { ConfigService } from './services/ConfigService';
export { WIDGET_STYLES } from './constants/styles';
export { SIMPLE_ICONS, ANIMATED_BOT } from './constants/icons';
export * from './types/config';

export interface CluaizWidgetInstance {
    launcher: Launcher;
    chatWindow: ChatWindow;
    configService: ConfigService;
}

export function initCluaizWidget(options?: { orgId?: string; baseUrl?: string }): CluaizWidgetInstance | null {
    if (typeof document === 'undefined') return null;

    // ── Inject styles ──
    if (!document.getElementById('cluaiz-widget-styles')) {
        const s = document.createElement('style');
        s.id = 'cluaiz-widget-styles';
        s.innerHTML = WIDGET_STYLES;
        document.head.appendChild(s);
    }

    // ── Read orgId ──
    const scriptTag = document.querySelector('script[data-org-id]') as HTMLScriptElement | null;
    const searchParams = typeof window !== 'undefined' ? new URLSearchParams(window.location.search) : new URLSearchParams();
    const orgId: string = options?.orgId || scriptTag?.getAttribute('data-org-id') || (window as any).CLUAIZ_ORG_ID || searchParams.get('orgId') || 'default-cluaiz';

    if (options?.baseUrl && typeof window !== 'undefined') {
        (window as any).CLUAIZ_EMBED_URL = options.baseUrl;
    }

    // ── Init modules ──
    const launcher = new Launcher();
    const chatWindow = new ChatWindow(orgId, launcher);
    const configService = new ConfigService(orgId, (config: any) => {
        const cfg = config.widgetConfig || config;
        if (cfg.icon && cfg.theme) {
            launcher.applyConfig(cfg);
            chatWindow.updatePosition();
        }
    });

    configService.fetchConfig();
    configService.startHeartbeat();
    configService.initEventListeners();

    // ── Restore notification persistence ──
    let unreadCount = parseInt(localStorage.getItem('cluaiz_unread_count') || '0');
    let lastMsg = localStorage.getItem('cluaiz_last_msg') || '';

    if (unreadCount > 0) {
        setTimeout(() => {
            launcher.toggleNotification(unreadCount);
            if (lastMsg) launcher.showPreview(lastMsg);
        }, 1500);
    }

    // ── New message handler ──
    window.addEventListener('message', (e) => {
        if (e.data?.type === 'CLUAIZ_NEW_MESSAGE' && !chatWindow.isChatOpen()) {
            unreadCount += e.data.count || 1;
            lastMsg = e.data.text || lastMsg;
            localStorage.setItem('cluaiz_unread_count', String(unreadCount));
            localStorage.setItem('cluaiz_last_msg', lastMsg);
            launcher.toggleNotification(unreadCount);
            if (e.data.text) launcher.showPreview(e.data.text);
            try {
                const snd = new Audio('https://cdn.freesound.org/previews/536/536108_11532476-lq.mp3');
                snd.volume = 0.5;
                snd.play().catch(() => { });
            } catch { }
        }
    });

    // ── Click handler ──
    launcher.element.addEventListener('click', () => {
        if (launcher.clickWasDrag) { launcher.clickWasDrag = false; return; }

        chatWindow.toggle();
        if (chatWindow.isChatOpen()) {
            unreadCount = 0; lastMsg = '';
            localStorage.removeItem('cluaiz_unread_count');
            localStorage.removeItem('cluaiz_last_msg');
            launcher.toggleNotification(0);
            setTimeout(() => chatWindow.updatePosition(), 50);
        }
    });

    // ── Storage sync ──
    window.addEventListener('storage', (e) => {
        if (e.key === 'cluaiz-widget-config') configService.fetchConfig();
    });

    // ── Restore sidebar open state ──
    if (chatWindow.isSidebar && chatWindow.isOpen) chatWindow.toggle(true);

    return { launcher, chatWindow, configService };
}

// Auto-run if script is embedded directly with data-auto-init
if (typeof window !== 'undefined' && typeof document !== 'undefined') {
    const script = document.querySelector('script[data-auto-init="true"]');
    if (script) {
        initCluaizWidget();
    }
}
