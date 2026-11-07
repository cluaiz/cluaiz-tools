// ChatWindow.ts — Resizable, dockable chat window supporting floating and sidebar modes

export class ChatWindow {
    iframe: HTMLIFrameElement | null = null;
    resizer: HTMLElement | null = null;
    handles: Record<string, HTMLElement> = {};
    isOpen: boolean;
    isSidebar: boolean;
    side: string;
    isCollapsed = false;
    orgId: string;
    launcher: any;
    currentWidth: number;
    currentHeight: number;

    constructor(orgId: string, launcher: any) {
        this.orgId = orgId;
        this.launcher = launcher;
        this.currentWidth = parseInt(localStorage.getItem('cluaiz_width') || '380');
        this.currentHeight = parseInt(localStorage.getItem('cluaiz_height') || '600');
        this.side = localStorage.getItem('cluaiz_side') || 'right';
        this.isSidebar = localStorage.getItem('cluaiz_mode') === 'sidebar';
        this.isOpen = localStorage.getItem('cluaiz_is_open') === 'true';
        this.initMessageListener();
        
        // Preload iframe immediately
        setTimeout(() => {
            if (!this.iframe && typeof document !== 'undefined') this.createIframe();
        }, 100);
    }

    toggle(t?: boolean) {
        this.isOpen = t !== undefined ? t : !this.isOpen;
        if (this.isOpen) {
            if (!this.iframe) this.createIframe();
            this.iframe?.classList.add('cluaiz-visible');
            localStorage.setItem('cluaiz_is_open', 'true');
            if (this.isSidebar) {
                this.toggleSidebar(true, this.side);
                this.collapseSidebar(false);
            } else {
                this.updatePosition();
            }
        } else {
            localStorage.setItem('cluaiz_is_open', 'false');
            if (this.isSidebar) {
                this.collapseSidebar(true);
            } else {
                this.iframe?.classList.remove('cluaiz-visible');
                this.toggleSidebar(false);
            }
        }
        this.syncHandles();
        if (this.isOpen) setTimeout(() => this.syncHandles(), 300);
    }

    isChatOpen() { return this.isOpen; }

    createIframe() {
        if (typeof document === 'undefined') return;
        this.iframe = document.createElement('iframe');
        this.iframe.id = 'cluaiz-iframe';
        this.iframe.setAttribute('allow', 'microphone; camera; clipboard-write;');
        const autoAnswer = typeof window !== 'undefined' && new URLSearchParams(window.location.search).get('auto_answer') === '1' ? '&auto_answer=1' : '';
        const defaultBaseUrl = typeof window !== 'undefined' && window.location.origin && window.location.origin !== 'null' && !window.location.origin.startsWith('file:') 
            ? window.location.origin 
            : 'http://localhost:8000';
        const baseUrl = (typeof window !== 'undefined' && (window as any).CLUAIZ_EMBED_URL) || defaultBaseUrl;
        this.iframe.src = `${baseUrl}/embed/chat?embed=1&orgId=${this.orgId}${autoAnswer}`;
        this.iframe.onload = () => {
            this.iframe?.contentWindow?.postMessage({ type: 'CLUAIZ_CONTEXT', url: window.location.href, title: document.title }, '*');
            setTimeout(() => this.syncHandles(), 100);
        };
        document.body.appendChild(this.iframe);
        if (!this.resizer) this.createResizer();
        this.updatePosition();
    }

    createResizer() {
        if (typeof document === 'undefined') return;
        this.resizer = document.createElement('div');
        this.resizer.id = 'cluaiz-resizer';
        this.resizer.innerHTML = `<div id="cluaiz-resizer-handle"><svg viewBox="0 0 24 24"><path d="M15.41 16.59L10.83 12l4.58-4.59L14 6l-6 6 6 6 1.41-1.41z"/></svg></div>`;
        document.body.appendChild(this.resizer);
        const handle = this.resizer.querySelector('#cluaiz-resizer-handle') as HTMLElement;
        if (handle) {
            handle.onclick = (e) => { e.stopPropagation(); this.collapseSidebar(true); };
        }

        this.handles = {
            r: this.createHandle('cluaiz-resizer-r'),
            l: this.createHandle('cluaiz-resizer-l'),
            t: this.createHandle('cluaiz-resizer-t'),
            b: this.createHandle('cluaiz-resizer-b'),
        };
        if (handle) this.initResizing(handle);
    }

    createHandle(className: string): HTMLElement {
        const h = document.createElement('div');
        h.className = `cluaiz-resizer ${className}`;
        h.style.display = 'none';
        document.body.appendChild(h);
        return h;
    }

    initResizing(handle: HTMLElement) {
        let activeHandle: string | null = null;
        let startX = 0, startY = 0, startW = 0, startH = 0, startTop = 0, startLeft = 0;

        const onMouseDown = (e: MouseEvent, type: string) => {
            e.preventDefault();
            activeHandle = type;
            startX = e.clientX; startY = e.clientY;
            startW = this.currentWidth; startH = this.currentHeight;
            const rect = this.iframe!.getBoundingClientRect();
            startTop = rect.top; startLeft = rect.left;
            this.iframe!.style.transition = 'none';
            this.iframe!.style.pointerEvents = 'none';
            document.body.classList.add('cluaiz-resizing');
            document.addEventListener('mousemove', onMouseMove);
            document.addEventListener('mouseup', onMouseUp);
        };

        const onMouseMove = (e: MouseEvent) => {
            if (!activeHandle) return;
            const dx = e.clientX - startX, dy = e.clientY - startY;
            const maxW = window.innerWidth * 0.4, maxH = window.innerHeight * 0.95;
            if (activeHandle === 'r') {
                this.currentWidth = Math.max(320, Math.min(maxW, startW + dx));
            } else if (activeHandle === 'l') {
                const newW = Math.max(320, Math.min(maxW, startW - dx));
                if (newW !== this.currentWidth) {
                    this.currentWidth = newW;
                    if (!this.isSidebar) this.iframe!.style.left = `${startLeft + (startW - newW)}px`;
                }
            } else if (!this.isSidebar && activeHandle === 'b') {
                this.currentHeight = Math.max(300, Math.min(maxH, startH + dy));
            } else if (!this.isSidebar && activeHandle === 't') {
                const newH = Math.max(300, Math.min(maxH, startH - dy));
                if (newH !== this.currentHeight) {
                    this.currentHeight = newH;
                    this.iframe!.style.top = `${startTop + (startH - newH)}px`;
                }
            }
            if (this.isSidebar) {
                this.applySidebarStyles();
            } else {
                this.iframe!.style.width = `${this.currentWidth}px`;
                this.iframe!.style.height = `${this.currentHeight}px`;
                this.syncHandles();
            }
        };

        const onMouseUp = () => {
            activeHandle = null;
            this.iframe!.style.transition = '';
            this.iframe!.style.pointerEvents = 'all';
            document.body.classList.remove('cluaiz-resizing');
            localStorage.setItem('cluaiz_width', String(this.currentWidth));
            localStorage.setItem('cluaiz_height', String(this.currentHeight));
            document.removeEventListener('mousemove', onMouseMove);
            document.removeEventListener('mouseup', onMouseUp);
        };

        this.handles.r.onmousedown = (e: MouseEvent) => onMouseDown(e, 'r');
        this.handles.l.onmousedown = (e: MouseEvent) => onMouseDown(e, 'l');
        this.handles.t.onmousedown = (e: MouseEvent) => onMouseDown(e, 't');
        this.handles.b.onmousedown = (e: MouseEvent) => onMouseDown(e, 'b');

        if (this.resizer) {
            this.resizer.onmousedown = (e: MouseEvent) => {
                if (e.target === handle || handle.contains(e.target as Node)) return;
                onMouseDown(e, this.side === 'right' ? 'l' : 'r');
            };
        }
    }

    syncHandles() {
        if (!this.iframe || this.isSidebar || !this.isOpen) {
            Object.values(this.handles).forEach(h => h.style.display = 'none');
            return;
        }
        const rect = this.iframe.getBoundingClientRect();
        const buffer = 4, offset = 22;
        Object.assign(this.handles.r.style, { display: 'block', left: `${rect.right - buffer}px`, top: `${rect.top + offset}px`, height: `${rect.height - offset * 2}px`, width: `${buffer * 2}px` });
        Object.assign(this.handles.l.style, { display: 'block', left: `${rect.left - buffer}px`, top: `${rect.top + offset}px`, height: `${rect.height - offset * 2}px`, width: `${buffer * 2}px` });
        Object.assign(this.handles.t.style, { display: 'block', top: `${rect.top - buffer}px`, left: `${rect.left + offset}px`, width: `${rect.width - offset * 2}px`, height: `${buffer * 2}px` });
        Object.assign(this.handles.b.style, { display: 'block', top: `${rect.bottom - buffer}px`, left: `${rect.left + offset}px`, width: `${rect.width - offset * 2}px`, height: `${buffer * 2}px` });
    }

    collapseSidebar(collapsed: boolean) {
        this.isCollapsed = collapsed;
        if (collapsed) {
            this.isOpen = false;
            localStorage.setItem('cluaiz_is_open', 'false');
            if (this.iframe) {
                this.iframe.classList.remove('cluaiz-visible');
                this.iframe.style.transform = `translateX(${this.side === 'right' ? '100%' : '-100%'})`;
                this.iframe.style.opacity = '0';
                this.iframe.style.pointerEvents = 'none';
            }
            if (this.resizer) this.resizer.classList.remove('active');
            document.body.classList.remove('cluaiz-squeezed');
            document.body.style.paddingRight = '0';
            document.body.style.paddingLeft = '0';
            const el = this.launcher.element;
            if (el) {
                el.style.transition = 'opacity 0.25s ease';
                el.style.opacity = '1';
                el.style.pointerEvents = 'all';
            }
        } else {
            if (this.iframe) {
                this.iframe.style.transform = 'none';
                this.iframe.style.opacity = '1';
                this.iframe.style.pointerEvents = 'all';
            }
            if (this.resizer) this.resizer.classList.add('active');
            document.body.classList.add('cluaiz-squeezed');
            this.applySidebarStyles();
            const el2 = this.launcher.element;
            if (el2) {
                el2.style.transition = 'opacity 0.25s ease';
                el2.style.opacity = '0';
                el2.style.pointerEvents = 'none';
            }
        }
    }

    updatePosition() {
        if (!this.iframe || this.isSidebar || !this.launcher?.element) return;
        const launcherRect = this.launcher.element.getBoundingClientRect();
        const iframeW = this.currentWidth, iframeH = this.currentHeight;
        const gap = 12, margin = 10;
        this.iframe.style.width = `${iframeW}px`;
        this.iframe.style.height = `${iframeH}px`;

        const spaceRight = window.innerWidth - launcherRect.right;
        const spaceLeft = launcherRect.left;
        let left: number;

        if (spaceRight >= iframeW + gap + margin) {
            left = launcherRect.right + gap;
        } else if (spaceLeft >= iframeW + gap + margin) {
            left = launcherRect.left - iframeW - gap;
        } else {
            left = Math.max(margin, Math.min(window.innerWidth - iframeW - margin, launcherRect.left + launcherRect.width / 2 - iframeW / 2));
        }

        let top = launcherRect.bottom - iframeH;
        if (top < margin) top = margin;
        if (top + iframeH > window.innerHeight - margin) top = window.innerHeight - iframeH - margin;

        this.iframe.style.left = `${left}px`;
        this.iframe.style.top = `${top}px`;
        this.iframe.style.right = 'auto';
        this.iframe.style.bottom = 'auto';
        this.syncHandles();
    }

    toggleSidebar(enabled: boolean, side = 'right') {
        if (!this.iframe) return;
        this.isSidebar = enabled; this.side = side;
        if (enabled && this.isOpen) {
            this.iframe.classList.add('cluaiz-sidebar-mode');
            this.collapseSidebar(false);
        } else {
            this.iframe.classList.remove('cluaiz-sidebar-mode');
            if (this.resizer) this.resizer.classList.remove('active');
            document.body.classList.remove('cluaiz-squeezed');
            document.body.style.paddingRight = '0';
            document.body.style.paddingLeft = '0';
            this.iframe.style.width = '380px'; this.iframe.style.height = '600px';
            this.iframe.style.transform = 'none';
            this.iframe.style.opacity = '';
            this.iframe.style.pointerEvents = '';
            this.updatePosition();
            this.iframe.style.borderRadius = '24px';
        }
    }

    applySidebarStyles() {
        if (!this.iframe || !this.resizer) return;
        const w = this.currentWidth + 'px';
        this.iframe.style.width = w; this.iframe.style.height = '100vh';
        this.iframe.style.top = '0'; this.iframe.style.bottom = 'auto';
        this.iframe.style.borderRadius = '0';
        const svg = this.resizer.querySelector('svg') as SVGElement | null;
        if (this.side === 'right') {
            this.iframe.style.right = '0'; this.iframe.style.left = 'auto';
            this.resizer.style.right = w; this.resizer.style.left = 'auto';
            this.resizer.style.top = '0'; this.resizer.style.height = '100vh';
            document.body.style.paddingRight = w; document.body.style.paddingLeft = '0';
            if (svg) svg.style.transform = 'rotate(180deg)';
        } else {
            this.iframe.style.left = '0'; this.iframe.style.right = 'auto';
            this.resizer.style.left = w; this.resizer.style.right = 'auto';
            this.resizer.style.top = '0'; this.resizer.style.height = '100vh';
            document.body.style.paddingLeft = w; document.body.style.paddingRight = '0';
            if (svg) svg.style.transform = 'rotate(0deg)';
        }
    }

    initMessageListener() {
        if (typeof window === 'undefined') return;
        window.addEventListener('message', (t) => {
            if (t.data === 'CLUAIZ_CLOSE') this.toggle(false);
            if (t.data?.type === 'CLUAIZ_SIDEBAR_TOGGLE') {
                this.isSidebar = t.data.enabled;
                localStorage.setItem('cluaiz_mode', this.isSidebar ? 'sidebar' : 'floating');
                this.toggleSidebar(this.isSidebar, 'right');
            }
            if (t.data?.type === 'CLUAIZ_READING') {
                document.querySelectorAll('iframe').forEach(frame => {
                    if (frame.contentWindow) frame.contentWindow.postMessage({ type: 'CLUAIZ_READING', isReading: t.data.isReading }, '*');
                });
            }
        });
        window.addEventListener('cluaiz-sidebar-mode', (e: any) => {
            this.isSidebar = true;
            this.side = e.detail?.side || 'right';
            localStorage.setItem('cluaiz_side', this.side);
            const chatActuallyOpen = this.isOpen && this.iframe && this.iframe.classList.contains('cluaiz-visible');
            if (chatActuallyOpen) this.toggleSidebar(true, this.side);
        });
        window.addEventListener('cluaiz-floating-mode', () => {
            this.isSidebar = false;
            this.toggleSidebar(false);
        });
    }
}
