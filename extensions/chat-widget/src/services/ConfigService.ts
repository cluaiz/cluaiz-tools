import { WidgetConfig } from '../types/config';

export class ConfigService {
    private apiUrl: string;
    private orgId: string;
    private onConfigUpdate: (config: WidgetConfig) => void;

    constructor(orgId: string, onUpdate: (config: WidgetConfig) => void) {
        this.orgId = orgId;
        this.onConfigUpdate = onUpdate;
        const isLocal = typeof window !== 'undefined' && (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1');
        this.apiUrl = isLocal ? 'http://localhost:8000/v1' : 'https://api.cluaiz.com/v1';
    }

    async fetchConfig() {
        try {
            const response = await fetch(`${this.apiUrl}/bots/default_bot/config`, {
                headers: {
                    'x-org-id': this.orgId
                }
            });
            const data = await response.json();
            if (data?.widgetConfig) {
                this.onConfigUpdate(data.widgetConfig);
            }
        } catch (error) {
            console.error('❌ Cluaiz Widget: Failed to fetch config', error);
        }
    }

    async sendHeartbeat() {
        try {
            await fetch(`${this.apiUrl}/analytics/heartbeat`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    orgId: this.orgId,
                    url: window.location.href,
                    domain: window.location.hostname
                })
            });
        } catch (e) { }
    }

    initEventListeners() {
        if (typeof window === 'undefined') return;

        window.addEventListener('cluaiz-config-update', (event: any) => {
            console.log('🔄 Cluaiz: Config update received', event.detail);
            if (event.detail) this.onConfigUpdate(event.detail);
            else this.fetchConfig();
        });

        window.addEventListener('storage', (event) => {
            if (event.key === 'cluaiz-widget-config') {
                this.fetchConfig();
            }
        });
    }

    startHeartbeat() {
        this.sendHeartbeat();
        setInterval(() => this.sendHeartbeat(), 5 * 60 * 1000);
    }
}
