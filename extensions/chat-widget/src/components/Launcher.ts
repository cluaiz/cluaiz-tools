// Launcher.ts — Floating widget launcher button with robot animation and drag-and-drop snapping
import { SIMPLE_ICONS, ANIMATED_BOT } from '../constants/icons';

function triggerAnim(el: HTMLElement | null | undefined, anim: string, dur = 1000) {
    if (!el) return;
    el.style.animation = 'none';
    el.offsetWidth; // trigger reflow
    el.style.animation = `${anim} ${dur}ms ease-in-out`;
    setTimeout(() => { if (el) el.style.animation = ''; }, dur);
}

export class Launcher {
    element!: HTMLElement;
    snapShadow!: HTMLElement;
    config: any = null;
    botElements: Record<string, HTMLElement | null> = {};
    mouseX = 0; mouseY = 0;
    rafId: number | null = null;
    currentUnreadCount = 0;
    lastPreviewText = '';
    notificationIcon: HTMLElement | undefined;
    isDragging = false; clickWasDrag = false;
    dragStartX = 0; dragStartY = 0; originalX = 0; originalY = 0;
    currentPos: { x: number; y: number } | null = JSON.parse(localStorage.getItem('cluaiz_pos') || 'null');
    isSidebar: boolean = localStorage.getItem('cluaiz_mode') === 'sidebar';
    side: string = localStorage.getItem('cluaiz_side') || 'right';
    isInteractionExpanded = false;

    handleMouseMove = (t: MouseEvent) => {
        this.mouseX = t.clientX;
        this.mouseY = t.clientY;
        if (!this.rafId) this.rafId = requestAnimationFrame(this.updateAnimation);
    };

    updateAnimation = () => {
        this.rafId = null;
        if (!this.element) return;
        const t = this.element.getBoundingClientRect();
        const e = t.left + t.width / 2, i = t.top + t.height / 2;
        const o = this.mouseX - e, a = this.mouseY - i;
        const r = Math.sqrt(o * o + a * a);

        // Broadcast to all iframes
        document.querySelectorAll('iframe').forEach((frame: HTMLIFrameElement) => {
            if (frame.contentWindow) {
                frame.contentWindow.postMessage({ type: 'CLUAIZ_EXTERNAL_MOUSE', x: o, y: a, r }, '*');
            }
        });

        const n = this.getScaling();
        const c = 100 * n.factor, l = 1.2 * n.factor;
        const p = r < t.width / 2 + c, m = r < t.width / 2;

        document.querySelectorAll('iframe').forEach((frame: HTMLIFrameElement) => {
            if (frame.contentWindow) {
                // Only send magnet hover if interaction menu is not expanded
                frame.contentWindow.postMessage({ type: 'CLUAIZ_MAGNET_HOVER', isHovered: this.isInteractionExpanded ? false : p }, '*');
            }
        });

        Array.from(this.element.children).forEach((d: any) => {
            if (d.classList?.contains('cluaiz-notification-magnet') || d.tagName === 'DIV' || d.tagName === 'svg') {
                if (p && !this.isInteractionExpanded) {
                    const h = o / l, y = a / l, b = m ? 1.05 : 1;
                    d.style.transition = 'transform 0.4s cubic-bezier(0.25, 1, 0.5, 1)';
                    d.style.transform = `translate3d(${h}px, ${y}px, 0) scale(${b})`;
                } else {
                    d.style.transition = 'transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1)';
                    d.style.transform = 'translate3d(0, 0, 0) scale(1)';
                }
            }
        });

        if (this.config?.icon?.type === 'robot' && this.botElements.body) {
            const f = Math.min(r / 300, 1);
            const h = o / (r || 1) * f * 8, y = a / (r || 1) * f * 6;
            if (this.botElements.face) {
                this.botElements.face.style.setProperty('--offset-x', `${h * 0.4}px`);
                this.botElements.face.style.setProperty('--offset-y', `${y * 0.4}px`);
            }
            if (this.botElements.features) {
                this.botElements.features.style.setProperty('--offset-x', `${h * 1.2}px`);
                this.botElements.features.style.setProperty('--offset-y', `${y * 1.2}px`);
            }
            const M = o / (r || 1) * f * 4, E = a / (r || 1) * f * 4;
            this.element.style.setProperty('--pupil-x', `${M}px`);
            this.element.style.setProperty('--pupil-y', `${E}px`);
        }
    };

    notifyIframeHover = (isHovered: boolean) => {
        document.querySelectorAll('iframe').forEach((frame: HTMLIFrameElement) => {
            if (frame.contentWindow) {
                frame.contentWindow.postMessage({ type: 'CLUAIZ_EXTERNAL_HOVER', isHovered }, '*');
            }
        });
    };

    constructor() {
        this.element = document.createElement('div');
        this.element.id = 'cluaiz-launcher';
        document.body.appendChild(this.element);

        this.snapShadow = document.createElement('div');
        this.snapShadow.className = 'cluaiz-snap-shadow';
        document.body.appendChild(this.snapShadow);

        // Apply default config immediately so widget is visible before API responds
        this.applyConfig({
            theme: { primaryColor: '#3B82F6', secondaryColor: '#8B5CF6', useGradient: true, iconColor: '#FFFFFF', iconStyle: 'solid', showBox: true, boxRadius: 50, iconScale: 1, boxAnimation: 'none' },
            icon: { type: 'chat', size: 'md' },
            animation: { robot: 'float', simple: 'ripple', entrance: 'popup', speed: 1, intensity: 1 },
            position: { horizontal: 'right', vertical: 'bottom', offsetX: 20, offsetY: 20 },
            behavior: { robot: { hover: 'giggle', click: 'nod' }, icon: { hover: 'none', click: 'ripple-burst' } },
            robot: { eyeColor: '#3B82F6', cheekColor: '#F472B6', lipColor: '#F472B6', earColor: '#0F172A', eyebrowColor: '#F59E0B', bodyColor: '#FFFFFF', useGradient: false, gradientType: 'linear', gradientColor1: '#3B82F6', gradientColor2: '#8B5CF6', gradientAngle: 45, animateGradient: false },
            window: { theme: 'auto', colorMode: 'auto' }
        });

        this.initInteractionListeners();
        this.initDragLogic();

        window.addEventListener('cluaiz-sidebar-mode', (ev: any) => {
            this.isSidebar = true;
            this.side = ev.detail?.side || this.side || 'right';
        });
        window.addEventListener('cluaiz-floating-mode', () => {
            this.isSidebar = false;
            this.element.classList.remove('cluaiz-snapped');
            this.element.style.transition = 'opacity 0.25s ease';
            this.element.style.opacity = '1';
            this.element.style.pointerEvents = 'all';
        });
    }

    initDragLogic() {
        if (typeof window === 'undefined' || window.innerWidth < 768) return;
        const down = (e: MouseEvent) => {
            if (e.button !== 0) return;
            this.isDragging = false; this.clickWasDrag = false;
            this.dragStartX = e.clientX; this.dragStartY = e.clientY;
            const rect = this.element.getBoundingClientRect();
            this.originalX = rect.left; this.originalY = rect.top;
            this.element.style.transition = 'none';
            document.addEventListener('mousemove', move);
            document.addEventListener('mouseup', up);
        };
        const move = (e: MouseEvent) => {
            const dx = e.clientX - this.dragStartX, dy = e.clientY - this.dragStartY;
            if (!this.isDragging && Math.sqrt(dx * dx + dy * dy) < 5) return;
            if (!this.isDragging) {
                this.isDragging = true; this.clickWasDrag = true;
                this.element.classList.add('cluaiz-dragging');
                document.body.classList.add('cluaiz-dragging-active');
                document.querySelectorAll('iframe').forEach((f: HTMLIFrameElement) => f.contentWindow?.postMessage({ type: 'CLUAIZ_DRAG_STATE', dragging: true }, '*'));
            }
            const newX = Math.max(0, Math.min(window.innerWidth - this.element.offsetWidth, this.originalX + dx));
            const newY = Math.max(10, Math.min(window.innerHeight - this.element.offsetHeight - 10, this.originalY + dy));
            this.element.style.left = `${newX}px`; this.element.style.top = `${newY}px`;
            this.element.style.right = 'auto'; this.element.style.bottom = 'auto';

            const shadowWidth = (localStorage.getItem('cluaiz_width') || '380') + 'px';
            const shadowHeight = (localStorage.getItem('cluaiz_height') || '600') + 'px';
            const numShadowHeight = parseInt(shadowHeight), numShadowWidth = parseInt(shadowWidth);
            const clampedSnapTop = Math.max(10, Math.min(window.innerHeight - numShadowHeight - 10, newY));

            if (!this.isSidebar) {
                if (newX < 15) {
                    this.snapShadow.classList.add('visible'); this.snapShadow.classList.remove('floating-preview');
                    this.snapShadow.style.left = '0'; this.snapShadow.style.right = 'auto';
                    this.snapShadow.style.width = shadowWidth; this.snapShadow.style.height = '100vh';
                    this.snapShadow.style.top = '0'; this.snapShadow.style.borderRadius = '0';
                } else if (newX > window.innerWidth - this.element.offsetWidth - 15) {
                    this.snapShadow.classList.add('visible'); this.snapShadow.classList.remove('floating-preview');
                    this.snapShadow.style.right = '0'; this.snapShadow.style.left = 'auto';
                    this.snapShadow.style.width = shadowWidth; this.snapShadow.style.height = '100vh';
                    this.snapShadow.style.top = '0'; this.snapShadow.style.borderRadius = '0';
                } else {
                    this.snapShadow.classList.remove('visible', 'floating-preview');
                }
            } else {
                const distAway = this.side === 'left' ? newX : (window.innerWidth - this.element.offsetWidth - newX);
                if (distAway > Math.max(150, window.innerWidth * 0.15)) {
                    this.snapShadow.classList.add('visible', 'floating-preview');
                    this.snapShadow.style.width = shadowWidth; this.snapShadow.style.top = `${clampedSnapTop}px`;
                    this.snapShadow.style.height = shadowHeight; this.snapShadow.style.borderRadius = '18px';
                    if (this.side === 'left') {
                        let lp = newX + this.element.offsetWidth + 10;
                        if (lp + numShadowWidth > window.innerWidth - 10) lp = window.innerWidth - numShadowWidth - 10;
                        this.snapShadow.style.left = `${lp}px`; this.snapShadow.style.right = 'auto';
                    } else {
                        let lp = newX - numShadowWidth - 10;
                        if (lp < 10) lp = 10;
                        this.snapShadow.style.left = `${lp}px`; this.snapShadow.style.right = 'auto';
                    }
                } else {
                    this.snapShadow.classList.add('visible'); this.snapShadow.classList.remove('floating-preview');
                    this.snapShadow.style.width = shadowWidth; this.snapShadow.style.height = '100vh';
                    this.snapShadow.style.top = '0'; this.snapShadow.style.borderRadius = '0';
                    if (this.side === 'left') { this.snapShadow.style.left = '0'; this.snapShadow.style.right = 'auto'; }
                    else { this.snapShadow.style.right = '0'; this.snapShadow.style.left = 'auto'; }
                }
            }
        };
        const up = (e: MouseEvent) => {
            const wasDragging = this.isDragging;
            this.isDragging = false;
            this.element.classList.remove('cluaiz-dragging');
            document.body.classList.remove('cluaiz-dragging-active');
            this.snapShadow.classList.remove('visible', 'floating-preview');
            this.snapShadow.style.height = '100vh';
            document.removeEventListener('mousemove', move); document.removeEventListener('mouseup', up);
            if (!wasDragging) return;
            document.querySelectorAll('iframe').forEach((f: HTMLIFrameElement) => f.contentWindow?.postMessage({ type: 'CLUAIZ_DRAG_STATE', dragging: false }, '*'));

            const rect = this.element.getBoundingClientRect();
            let finalX = rect.left, finalY = rect.top;
            if (!this.isSidebar) {
                if (rect.left < 15) { finalX = 10; this.isSidebar = true; this.side = 'left'; }
                else if (rect.left > window.innerWidth - this.element.offsetWidth - 15) { finalX = window.innerWidth - rect.width - 10; this.isSidebar = true; this.side = 'right'; }
            } else {
                const distAway = this.side === 'left' ? rect.left : (window.innerWidth - rect.width - rect.left);
                if (distAway > Math.max(150, window.innerWidth * 0.15)) { this.isSidebar = false; }
                else { finalX = this.side === 'left' ? 10 : window.innerWidth - rect.width - 10; }
            }
            this.element.style.transition = 'all 0.5s cubic-bezier(0.19, 1, 0.22, 1)';
            this.element.style.left = `${finalX}px`; this.element.style.top = `${finalY}px`;
            if (this.isSidebar) {
                localStorage.setItem('cluaiz_snap_y', String(finalY));
                localStorage.setItem('cluaiz_side', this.side);
            } else {
                localStorage.setItem('cluaiz_pos', JSON.stringify({ x: finalX, y: finalY }));
            }
            localStorage.setItem('cluaiz_mode', this.isSidebar ? 'sidebar' : 'floating');
            if (this.isSidebar) window.dispatchEvent(new CustomEvent('cluaiz-sidebar-mode', { detail: { side: this.side } }));
            else window.dispatchEvent(new CustomEvent('cluaiz-floating-mode'));
        };
        this.element.addEventListener('mousedown', down);
    }

    applyConfig(t: any) {
        this.config = t;
        if (t.icon?.type === 'robot') {
            this.element.innerHTML = ANIMATED_BOT;
            this.initRobotElements();
        } else {
            const i = t.icon?.type || 'chat', o = t.theme?.iconStyle || 'solid';
            let a: any = (SIMPLE_ICONS as any)[i];
            if (typeof a === 'object') a = a[o];
            this.element.innerHTML = a || SIMPLE_ICONS.chat;
        }
        this.initMouseTracking();
        this.applyStyles(t);
        this.applyAnimations(t);
        if (this.currentUnreadCount > 0) {
            const count = this.currentUnreadCount;
            this.currentUnreadCount = 0;
            this.toggleNotification(count);
        }
    }

    initRobotElements() {
        this.botElements = {
            body: document.getElementById('layer-body'),
            face: document.getElementById('layer-face'),
            features: document.getElementById('layer-features'),
            eyeGroupL: document.getElementById('eye-group-l'),
            eyeGroupR: document.getElementById('eye-group-r'),
            botWrapper: document.getElementById('bot-anim-wrapper'),
        };
    }

    applyStyles(t: any) {
        const e = this.element;
        if (t.position && !this.isSidebar) {
            const hor = t.position.horizontal || 'right', ver = t.position.vertical || 'bottom';
            const ox = t.position.offsetX ?? 20, oy = t.position.offsetY ?? 20;
            e.style.bottom = ver === 'bottom' ? `${oy}px` : 'auto';
            e.style.top = ver === 'top' ? `${oy}px` : 'auto';
            e.style.right = hor === 'right' ? `${ox}px` : 'auto';
            e.style.left = hor === 'left' ? `${ox}px` : 'auto';
        }
        if (this.isSidebar) {
            const snapY = parseFloat(localStorage.getItem('cluaiz_snap_y') || '-1');
            const snapSide = localStorage.getItem('cluaiz_side') || 'right';
            if (snapY >= 0) {
                e.style.top = `${snapY}px`; e.style.bottom = 'auto';
                e.style.left = snapSide === 'left' ? '10px' : 'auto';
                e.style.right = snapSide === 'right' ? '10px' : 'auto';
            }
        } else if (this.currentPos) {
            e.style.left = `${this.currentPos.x}px`;
            e.style.top = `${this.currentPos.y}px`;
            e.style.right = 'auto'; e.style.bottom = 'auto';
        }

        if (t.icon?.type === 'robot' && t.robot) {
            e.style.setProperty('--body-color', t.robot.bodyColor || '#FFFFFF');
            e.style.setProperty('--cluaiz-primary-color', t.robot.bodyColor || '#FFFFFF');
            e.style.setProperty('--eye-color', t.robot.eyeColor || '#3B82F6');
            e.style.setProperty('--lip-color', t.robot.lipColor || '#F472B6');
            e.style.setProperty('--cheek-color', t.robot.cheekColor || '#F472B6');
            e.style.setProperty('--ear-color', t.robot.earColor || '#0F172A');
            e.style.setProperty('--eyebrow-color', t.robot.eyebrowColor || '#F59E0B');
            if (t.robot.useGradient) {
                e.style.setProperty('--cluaiz-primary-color', t.robot.gradientColor1);
                e.style.setProperty('--gradient-color1', t.robot.gradientColor1);
                e.style.setProperty('--gradient-color2', t.robot.gradientColor2);
                e.style.setProperty('--gradient-angle', `${t.robot.gradientAngle}deg`);
                t.robot.animateGradient ? e.classList.add('gradient-animated') : e.classList.remove('gradient-animated');
                const r2 = (t.robot.gradientAngle - 90) * (Math.PI / 180);
                const gx1 = 50 + 40 * Math.cos(r2), gy1 = 50 + 40 * Math.sin(r2);
                const gx2 = 50 - 40 * Math.cos(r2), gy2 = 50 - 40 * Math.sin(r2);
                const hg = document.getElementById('head-gradient');
                if (hg) { hg.setAttribute('x1', `${gx1}%`); hg.setAttribute('y1', `${gy1}%`); hg.setAttribute('x2', `${gx2}%`); hg.setAttribute('y2', `${gx2}%`); }
            } else {
                e.style.setProperty('--gradient-color1', t.robot.bodyColor || '#FFFFFF');
                e.style.setProperty('--gradient-color2', t.robot.bodyColor || '#FFFFFF');
                e.classList.remove('gradient-animated');
            }
        } else if (t.theme) {
            const pColor = t.theme.primaryColor || '#3B82F6', sColor = t.theme.secondaryColor || pColor;
            e.style.setProperty('--cluaiz-primary-color', pColor);
            e.style.setProperty('--icon-color', t.theme.iconColor || '#FFFFFF');
            e.style.setProperty('--icon-scale', String(t.theme.iconScale || 1));
            e.style.setProperty('--primary-color', pColor);
            e.style.setProperty('--secondary-color', sColor);
            e.style.setProperty('--gradient-color1', pColor);
            e.style.setProperty('--gradient-color2', sColor);
            const isOutline = t.theme.iconStyle === 'outline';
            e.style.setProperty('--icon-fill', isOutline ? 'none' : (t.theme.iconColor || '#FFFFFF'));
            e.style.setProperty('--icon-stroke', isOutline ? (t.theme.iconColor || '#FFFFFF') : 'none');
            e.style.setProperty('--icon-stroke-width', isOutline ? '2px' : '0px');
            if (t.theme.showBox === false) {
                e.style.setProperty('--box-bg', 'transparent');
                e.style.setProperty('--box-shadow', 'none');
                e.style.setProperty('--box-border', 'none');
            } else {
                e.style.removeProperty('--box-bg');
                e.style.removeProperty('--box-shadow');
                e.style.removeProperty('--box-border');
            }
        }
        e.style.setProperty('--anim-speed', String(t.animation?.speed || 1));
        e.style.setProperty('--anim-intensity', String(t.animation?.intensity || 1));
        if (t.theme?.boxRadius !== undefined) {
            e.style.setProperty('--box-radius', `${t.theme.boxRadius}%`);
        }
        const sz: Record<string, string> = { xs: '60px', sm: '75px', md: '90px', lg: '105px', xl: '120px' };
        const o = sz[t.icon?.size || 'md'] || '90px';
        e.style.width = o; e.style.height = o;
    }

    applyAnimations(t: any) {
        const e = this.element;
        e.className = 'cluaiz-launcher';
        if (t.animation?.entrance) {
            e.classList.add(`entrance-${t.animation.entrance}`);
            setTimeout(() => e.classList.remove(`entrance-${t.animation.entrance}`), 1500);
        }
        if (this.isSidebar) e.classList.add('cluaiz-snapped');
        else e.classList.remove('cluaiz-snapped');
        e.classList.add(`box-anim-${t.theme?.boxAnimation || 'none'}`);
        e.classList.add(`icon-anim-${t.icon?.type === 'robot' ? 'none' : (t.animation?.simple || 'none')}`);
        if (t.icon?.type === 'robot') {
            const r = t.animation?.robot || 'float';
            const bw = this.botElements.botWrapper;
            if (bw) bw.className = `bot-container-anim robot-${r}`;
            this.startBlinking();
        }
    }

    startBlinking() {
        const blink = () => {
            this.element.querySelectorAll('.cluaiz-eye-blink').forEach((i: any) => {
                i.style.animation = 'none'; i.offsetWidth;
                i.style.animation = 'bot-blink 0.6s ease-in-out';
            });
            setTimeout(blink, 2000 + Math.random() * 3000);
        };
        setTimeout(blink, 2000);
    }

    initInteractionListeners() {
        this.element.addEventListener('mouseenter', () => {
            if (!this.config) return;
            this.element.classList.add('is-hovered');
            if (this.config.icon?.type === 'robot') {
                const hov = this.config.behavior?.robot?.hover || 'none';
                if (hov !== 'none') {
                    const anims: Record<string, string> = { wave: 'robot-wave 1s ease-in-out', surprise: 'robot-peep 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275)', giggle: 'robot-spin 0.8s linear' };
                    if (anims[hov] && this.botElements.botWrapper) (this.botElements.botWrapper as any).style.animation = anims[hov];
                }
            } else {
                const hov = this.config.behavior?.icon?.hover || 'none';
                this.element.classList.remove('hover-effect-scale', 'hover-effect-tilt');
                if (hov === 'scale') this.element.classList.add('hover-effect-scale');
                else if (hov === 'tilt') this.element.classList.add('hover-effect-tilt');
                else if (hov === 'bounce') this.element.style.animation = 'icon-bounce 0.6s ease-in-out infinite';
                else if (hov === 'pulse') this.element.style.animation = 'icon-pulse 1.5s ease-in-out infinite';
            }
            if (this.config.icon?.type === 'robot' || this.config.icon?.type === 'cluaiz3bairobot') {
                this.notifyIframeHover(true);
            }
        });
        this.element.addEventListener('mouseleave', () => {
            this.element.classList.remove('is-hovered');
            this.element.style.animation = '';
            if (this.botElements.botWrapper) (this.botElements.botWrapper as any).style.animation = '';
            if (this.config?.icon?.type === 'robot' || this.config?.icon?.type === 'cluaiz3bairobot') {
                this.notifyIframeHover(false);
            }
        });
        this.element.addEventListener('mousedown', () => {
            if (!this.config) return;
            const isRobot = this.config.icon?.type === 'robot';
            const act = isRobot ? this.config.behavior?.robot?.click || 'nod' : this.config.behavior?.icon?.click || 'none';
            if (act !== 'none') this.element.classList.add('is-active');
            this.handleInteraction(act, isRobot);
        });
        this.element.addEventListener('mouseup', () => this.element.classList.remove('is-active'));

        const setExpanded = (open: boolean, skipNotify = false) => {
            this.isInteractionExpanded = open;

            // Manage Parent CSS Transform (Scaling)
            if (open) {
                this.element.style.transform = 'scale(4.5)';
                this.element.style.transformOrigin = 'center center';
                this.element.style.zIndex = '999999';
                this.element.style.width = '100px';
                this.element.style.height = '100px';
                this.element.style.borderRadius = '';
                this.element.style.transition = 'transform 0.4s cubic-bezier(0.16, 1, 0.3, 1)';

                const iframe = this.element.querySelector('iframe');
                if (iframe) iframe.style.pointerEvents = 'auto';
            } else {
                this.element.style.transform = '';
                this.element.style.zIndex = '2147483647';
                this.element.style.width = '100px';
                this.element.style.height = '100px';
                this.element.style.borderRadius = '';
                this.element.style.transition = 'transform 0.4s cubic-bezier(0.16, 1, 0.3, 1)';

                const iframe = this.element.querySelector('iframe');
                if (iframe) iframe.style.pointerEvents = 'none';
            }

            // Sync with iframe (Open/Close menu inside)
            if (!skipNotify) {
                document.querySelectorAll('iframe').forEach((f: HTMLIFrameElement) => {
                    f.contentWindow?.postMessage({
                        type: 'CLUAIZ_DOUBLE_CLICK',
                        forceState: open
                    }, '*');
                });
            }
        };

        window.addEventListener('message', (e) => {
            if (e.data?.type === 'CLUAIZ_MENU_STATE') {
                const shouldBeOpen = !!e.data.isOpen;
                if (shouldBeOpen !== this.isInteractionExpanded) {
                    setExpanded(shouldBeOpen, true);
                }
            }
        });

        // Double Click
        this.element.addEventListener('dblclick', (e) => {
            e.preventDefault();
            setExpanded(!this.isInteractionExpanded);
        });

        // Right Click (contextmenu)
        this.element.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            setExpanded(!this.isInteractionExpanded);
        });
    }

    handleInteraction(t: string, isRobot: boolean) {
        if (isRobot) {
            const eyes = this.element.querySelectorAll('.cluaiz-eye-blink') as NodeListOf<HTMLElement>;
            if (t === 'jump') triggerAnim(this.botElements.botWrapper as HTMLElement, 'robot-bounce', 500);
            else if (t === 'nod') triggerAnim(this.element.querySelector('.robot-body') as HTMLElement, 'bot-nod', 600);
            else if (t === 'wink' && eyes.length >= 2) { eyes[1].style.transform = 'scaleY(0.1)'; setTimeout(() => { eyes[1].style.transform = 'scaleY(1)'; }, 300); }
            else if (t === 'close-eyes') eyes.forEach(o => { o.style.transform = 'scaleY(0.1)'; setTimeout(() => { o.style.transform = 'scaleY(1)'; }); });
        } else if (t === 'ripple-burst') {
            const rip = document.createElement('div');
            rip.className = 'cluaiz-ripple';
            Object.assign(rip.style, { position: 'absolute', top: '50%', left: '50%', width: '100%', height: '100%', borderRadius: '50%', border: '3px solid rgba(255,255,255,0.9)', transform: 'translate(-50%, -50%) scale(0.4)', opacity: '1', pointerEvents: 'none', transition: 'all 0.5s cubic-bezier(0.16, 1, 0.3, 1)', boxShadow: '0 0 20px rgba(255,255,255,0.5)' });
            this.element.appendChild(rip);
            requestAnimationFrame(() => { rip.style.transform = 'translate(-50%, -50%) scale(2.8)'; rip.style.opacity = '0'; rip.style.borderWidth = '0px'; });
            setTimeout(() => rip.remove(), 600);
        } else if (t === 'shake') triggerAnim(this.element, 'icon-shake', 500);
        else if (t === 'flip') triggerAnim(this.element, 'icon-flip', 800);
        else if (t === 'pop') { this.element.style.transform = 'scale(0.85)'; setTimeout(() => { this.element.style.transform = ''; }, 150); }
    }

    initMouseTracking() {
        if (typeof window === 'undefined') return;
        this.mouseX = window.innerWidth / 2; this.mouseY = window.innerHeight / 2;
        document.addEventListener('mousemove', this.handleMouseMove, { passive: true });
    }

    getScaling() {
        const sizes: Record<string, number> = { xs: 60, sm: 75, md: 90, lg: 105, xl: 120 };
        const i = (sizes[this.config?.icon?.size || 'md'] || 90) / 90;
        return { factor: i, bellSize: Math.round(42 * i), iconSize: Math.round(22 * i), badgeSize: Math.round(20 * i), badgeFont: Math.round(11 * i), offset: -Math.round(12 * i) };
    }

    toggleNotification(t: number) {
        this.currentUnreadCount = t;
        const e = this.config?.theme?.primaryColor || '#3B82F6';
        const isBottom = this.config?.position?.vertical !== 'top';
        const isRight = this.config?.position?.horizontal !== 'left';
        const a = this.getScaling();
        if (t > 0) {
            if (!this.notificationIcon) {
                this.notificationIcon = document.createElement('div');
                this.notificationIcon.className = 'cluaiz-notification-magnet';
                Object.assign(this.notificationIcon.style, {
                    position: 'absolute',
                    [isBottom ? 'top' : 'bottom']: `${a.offset}px`,
                    [isRight ? 'right' : 'left']: `${a.offset}px`,
                    width: `${a.bellSize}px`, height: `${a.bellSize}px`, borderRadius: '50%',
                    backgroundColor: 'white', boxShadow: `0 ${8 * a.factor}px ${20 * a.factor}px rgba(0,0,0,0.12), 0 0 ${15 * a.factor}px ${e}25`,
                    display: 'flex', alignItems: 'center', justifyContent: 'center',
                    cursor: 'pointer', zIndex: '20', transition: 'all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275)',
                    animation: 'cluaiz-pop 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275)',
                    border: '1.5px solid rgba(255,255,255,0.8)',
                });
                this.notificationIcon.innerHTML = `<svg width="${a.iconSize}" height="${a.iconSize}" viewBox="0 0 24 24" fill="none" stroke="${e}" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"></path><path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path></svg><div class="cluaiz-bell-badge" style="position:absolute;top:-${Math.round(4 * a.factor)}px;right:-${Math.round(4 * a.factor)}px;background:${e};color:white;min-width:${a.badgeSize}px;height:${a.badgeSize}px;border-radius:${a.badgeSize / 2}px;font-size:${a.badgeFont}px;font-weight:900;display:flex;align-items:center;justify-content:center;padding:0 ${Math.round(4 * a.factor)}px;border:${2.3 * a.factor}px solid white;box-shadow:0 ${4 * a.factor}px ${12 * a.factor}px ${e}40;font-family:'Inter',sans-serif;"></div>`;
                this.notificationIcon.onmouseenter = () => { if (this.lastPreviewText) this.showPreview(this.lastPreviewText, true); this.notificationIcon!.style.transform = `scale(1.2) rotate(${isRight ? '12' : '-12'}deg)`; };
                this.notificationIcon.onmouseleave = () => { this.notificationIcon!.style.transform = 'scale(1)'; };
                this.element.appendChild(this.notificationIcon);
            }
            const badge = this.notificationIcon.querySelector('.cluaiz-bell-badge') as HTMLElement | null;
            if (badge) { badge.textContent = t > 99 ? '99+' : String(t); badge.style.display = 'flex'; }
            this.notificationIcon.style.display = 'flex';
            this.notificationIcon.style.transform = 'scale(1.25) rotate(-15deg)';
            setTimeout(() => { if (this.notificationIcon) this.notificationIcon.style.transform = ''; }, 250);
        } else {
            if (this.notificationIcon) { this.notificationIcon.remove(); this.notificationIcon = undefined; }
            this.lastPreviewText = '';
            const bubble = document.querySelector('.cluaiz-preview-bubble') as HTMLElement | null;
            if (bubble) bubble.style.opacity = '0';
        }
    }

    showPreview(t: string, sticky = false) {
        if (!t || typeof window === 'undefined') return;
        this.lastPreviewText = t;
        const existing = document.querySelector('.cluaiz-preview-bubble');
        if (existing) existing.remove();
        const i = document.createElement('div');
        i.className = 'cluaiz-preview-bubble';
        const o = this.config?.theme?.primaryColor || '#3B82F6';
        const isBottom = this.config?.position?.vertical !== 'top';
        const isRight = this.config?.position?.horizontal !== 'left';
        const n = this.getScaling();
        const c = this.element.getBoundingClientRect();
        const l = Math.round(20 * n.factor);
        const pos = isBottom ? { bottom: `${window.innerHeight - c.top + l}px` } : { top: `${c.bottom + l}px` };
        const hor = isRight ? { right: `${window.innerWidth - c.right + 10}px` } : { left: `${c.left + 10}px` };
        const d = Math.round(24 * n.factor), f = `${Math.round(14 * n.factor)}px ${Math.round(18 * n.factor)}px`;
        Object.assign(i.style, {
            position: 'fixed', ...pos, ...hor,
            backgroundColor: 'rgba(255,255,255,0.94)', backdropFilter: 'blur(16px) saturate(200%)',
            color: '#1e293b', padding: f, borderRadius: `${d}px`,
            borderBottomRightRadius: isBottom && isRight ? '4px' : `${d}px`, borderBottomLeftRadius: isBottom && !isRight ? '4px' : `${d}px`,
            borderTopRightRadius: !isBottom && isRight ? '4px' : `${d}px`, borderTopLeftRadius: !isBottom && !isRight ? '4px' : `${d}px`,
            boxShadow: `0 ${20 * n.factor}px ${40 * n.factor}px -${10 * n.factor}px rgba(0,0,0,0.15), 0 0 ${25 * n.factor}px ${o}15`,
            fontSize: `${Math.round(14 * n.factor)}px`, fontWeight: '600', maxWidth: `${Math.round(280 * n.factor)}px`,
            zIndex: '2147483647', opacity: '0', transform: `translateY(${isBottom ? '20' : '-20'}px) scale(0.9) rotate(${isRight ? '-2' : '2'}deg)`,
            transition: 'all 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275)', border: '1.5px solid rgba(255,255,255,0.7)',
            pointerEvents: 'all', cursor: 'pointer',
        });
        i.innerHTML = `<div style="display:flex;align-items:center;gap:${Math.round(8 * n.factor)}px;margin-bottom:${Math.round(8 * n.factor)}px;"><div style="width:${Math.round(8 * n.factor)}px;height:${Math.round(8 * n.factor)}px;background:${o};border-radius:50%;animation:cluaiz-pulse 1.5s infinite;"></div><div style="color:${o};font-size:${Math.round(11 * n.factor)}px;font-weight:800;text-transform:uppercase;letter-spacing:0.12em;">New Message</div></div><div style="line-height:1.6;overflow:hidden;display:-webkit-box;-webkit-line-clamp:4;-webkit-box-orient:vertical;font-family:'Outfit',sans-serif;">${t}</div>`;
        i.onclick = () => { this.element.click(); i.remove(); };
        document.body.appendChild(i);
        requestAnimationFrame(() => { i.style.opacity = '1'; i.style.transform = 'translateY(0) scale(1) rotate(0deg)'; });
        if (!sticky) {
            setTimeout(() => { if (i && document.body.contains(i)) { i.style.opacity = '0'; i.style.transform = `translateY(${isBottom ? '10' : '-10'}px) scale(0.95)`; setTimeout(() => i.remove(), 500); } }, 20000);
        } else {
            const hide = () => { if (i) { i.style.opacity = '0'; i.style.transform = `translateY(${isBottom ? '8' : '-8'}px) scale(0.98)`; setTimeout(() => i.remove(), 400); } };
            this.notificationIcon?.addEventListener('mouseleave', hide, { once: true });
        }
    }
}
