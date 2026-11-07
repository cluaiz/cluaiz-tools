/**
 * AssistantCanvas.ts
 * High-performance, zero-VRAM procedural 2D Canvas desktop companion avatar.
 * Features: Expression state machine, mouse tracking, lip-sync, physics drag & drop.
 */

import { AssistantVoiceSync, VoiceSyncState, MouthShape } from './AssistantVoiceSync';

export type AssistantExpression = 'idle' | 'thinking' | 'speaking' | 'listening' | 'happy' | 'surprised' | 'sleepy';

export interface AssistantTheme {
    bodyColor?: string;
    bodyAccentColor?: string;
    eyeColor?: string;
    pupilColor?: string;
    blushColor?: string;
    antennaColor?: string;
}

export interface AssistantOptions {
    container?: HTMLElement;
    size?: number; // Size in CSS pixels (e.g. 140)
    initialX?: number;
    initialY?: number;
    theme?: AssistantTheme;
    draggable?: boolean;
}

export class AssistantCanvas {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private container: HTMLElement;
    private size: number;
    private dpr: number;

    // Theme & styling
    private theme: Required<AssistantTheme>;

    // Expression & Animation State
    private expression: AssistantExpression = 'idle';
    private voiceSync: AssistantVoiceSync;
    private isSpeaking = false;
    private mouthShape: MouthShape = 'closed';
    private mouthOpen = 0;

    // Head bobbing & breathing
    private animTime = 0;
    private lastFrameTime = 0;
    private rafId: number | null = null;

    // Mouse Tracking
    private mouseX = 0;
    private mouseY = 0;
    private targetLookX = 0;
    private targetLookY = 0;
    private lookX = 0;
    private lookY = 0;

    // Blinking
    private blinkProgress = 0; // 0 = open, 1 = closed
    private isBlinking = false;
    private nextBlinkTimer: number | null = null;

    // Drag and Drop Physics
    private isDragging = false;
    private dragStartX = 0;
    private dragStartY = 0;
    private posX = 0;
    private posY = 0;
    private draggable: boolean;

    constructor(options: AssistantOptions = {}) {
        this.size = options.size || 140;
        this.draggable = options.draggable !== false;
        this.container = options.container || (typeof document !== 'undefined' ? document.body : (null as any));
        this.dpr = typeof window !== 'undefined' ? window.devicePixelRatio || 1 : 1;

        // Default Theme
        this.theme = {
            bodyColor: options.theme?.bodyColor || '#2563EB',
            bodyAccentColor: options.theme?.bodyAccentColor || '#3B82F6',
            eyeColor: options.theme?.eyeColor || '#FFFFFF',
            pupilColor: options.theme?.pupilColor || '#1E293B',
            blushColor: options.theme?.blushColor || 'rgba(244, 114, 182, 0.45)',
            antennaColor: options.theme?.antennaColor || '#F59E0B'
        };

        this.voiceSync = new AssistantVoiceSync();
        this.voiceSync.onStateChange((state: VoiceSyncState) => {
            this.isSpeaking = state.isSpeaking;
            this.mouthOpen = state.mouthOpen;
            this.mouthShape = state.mouthShape;
            if (state.isSpeaking && this.expression === 'idle') {
                this.expression = 'speaking';
            } else if (!state.isSpeaking && this.expression === 'speaking') {
                this.expression = 'idle';
            }
        });

        // Initialize Canvas
        if (typeof document !== 'undefined') {
            this.canvas = document.createElement('canvas');
            this.canvas.className = 'cluaiz-assistant-canvas';
            this.setupCanvasDOM(options.initialX, options.initialY);
            const context = this.canvas.getContext('2d');
            if (!context) throw new Error('Canvas 2D context is not available');
            this.ctx = context;

            this.initEventListeners();
            this.scheduleNextBlink();
            this.startLoop();
        } else {
            this.canvas = null as any;
            this.ctx = null as any;
        }
    }

    private setupCanvasDOM(initialX?: number, initialY?: number): void {
        this.canvas.width = this.size * this.dpr;
        this.canvas.height = this.size * this.dpr;
        this.canvas.style.width = `${this.size}px`;
        this.canvas.style.height = `${this.size}px`;
        this.canvas.style.position = 'fixed';
        this.canvas.style.zIndex = '2147483640';
        this.canvas.style.cursor = this.draggable ? 'grab' : 'default';
        this.canvas.style.userSelect = 'none';
        this.canvas.style.touchAction = 'none';

        // Load saved coordinates or defaults
        const savedPos = typeof localStorage !== 'undefined' ? JSON.parse(localStorage.getItem('cluaiz_assistant_pos') || 'null') : null;
        if (savedPos) {
            this.posX = savedPos.x;
            this.posY = savedPos.y;
        } else {
            this.posX = initialX ?? (typeof window !== 'undefined' ? window.innerWidth - this.size - 24 : 20);
            this.posY = initialY ?? (typeof window !== 'undefined' ? window.innerHeight - this.size - 24 : 20);
        }

        this.applyPosition();
        if (this.container) {
            this.container.appendChild(this.canvas);
        }
    }

    private applyPosition(): void {
        if (!this.canvas) return;
        this.canvas.style.left = `${this.posX}px`;
        this.canvas.style.top = `${this.posY}px`;
    }

    private initEventListeners(): void {
        if (typeof window === 'undefined') return;

        // Global mouse move for eye gaze tracking
        window.addEventListener('mousemove', (e: MouseEvent) => {
            this.mouseX = e.clientX;
            this.mouseY = e.clientY;

            const rect = this.canvas.getBoundingClientRect();
            const centerX = rect.left + rect.width / 2;
            const centerY = rect.top + rect.height / 2;

            const dx = this.mouseX - centerX;
            const dy = this.mouseY - centerY;
            const dist = Math.sqrt(dx * dx + dy * dy);

            // Normalized gaze (-1.0 to 1.0)
            const maxTrack = 250;
            const factor = Math.min(dist / maxTrack, 1);
            this.targetLookX = (dx / (dist || 1)) * factor;
            this.targetLookY = (dy / (dist || 1)) * factor;
        }, { passive: true });

        if (!this.draggable) return;

        // Drag handlers
        this.canvas.addEventListener('mousedown', (e: MouseEvent) => {
            if (e.button !== 0) return;
            this.isDragging = true;
            this.canvas.style.cursor = 'grabbing';
            this.dragStartX = e.clientX - this.posX;
            this.dragStartY = e.clientY - this.posY;
            e.preventDefault();
        });

        window.addEventListener('mousemove', (e: MouseEvent) => {
            if (!this.isDragging) return;
            const newX = Math.max(10, Math.min(window.innerWidth - this.size - 10, e.clientX - this.dragStartX));
            const newY = Math.max(10, Math.min(window.innerHeight - this.size - 10, e.clientY - this.dragStartY));
            this.posX = newX;
            this.posY = newY;
            this.applyPosition();
        });

        window.addEventListener('mouseup', () => {
            if (!this.isDragging) return;
            this.isDragging = false;
            this.canvas.style.cursor = 'grab';

            // Snap physics if close to window edge (< 25px)
            if (this.posX < 30) this.posX = 16;
            else if (this.posX > window.innerWidth - this.size - 30) this.posX = window.innerWidth - this.size - 16;
            this.applyPosition();

            if (typeof localStorage !== 'undefined') {
                localStorage.setItem('cluaiz_assistant_pos', JSON.stringify({ x: this.posX, y: this.posY }));
            }
        });
    }

    private scheduleNextBlink(): void {
        const delay = 2000 + Math.random() * 3500;
        this.nextBlinkTimer = typeof window !== 'undefined' ? window.setTimeout(() => {
            this.performBlink();
        }, delay) : null;
    }

    private performBlink(): void {
        this.isBlinking = true;
        const blinkDuration = 180;
        const start = performance.now();

        const animateBlink = (now: number) => {
            const elapsed = now - start;
            const progress = elapsed / blinkDuration;
            if (progress < 0.5) {
                this.blinkProgress = progress * 2;
            } else if (progress < 1) {
                this.blinkProgress = 2 - progress * 2;
            } else {
                this.blinkProgress = 0;
                this.isBlinking = false;
                this.scheduleNextBlink();
                return;
            }
            requestAnimationFrame(animateBlink);
        };
        requestAnimationFrame(animateBlink);
    }

    private startLoop(): void {
        this.lastFrameTime = performance.now();
        const render = (now: number) => {
            const dt = Math.min(0.1, (now - this.lastFrameTime) / 1000);
            this.lastFrameTime = now;
            this.animTime += dt;

            // Update Voice Sync & Physics
            this.voiceSync.update(dt);
            this.lookX += (this.targetLookX - this.lookX) * Math.min(1, dt * 10);
            this.lookY += (this.targetLookY - this.lookY) * Math.min(1, dt * 10);

            this.draw();
            this.rafId = requestAnimationFrame(render);
        };
        this.rafId = requestAnimationFrame(render);
    }

    /**
     * Render the assistant avatar on Canvas
     */
    private draw(): void {
        const ctx = this.ctx;
        const s = this.size * this.dpr;
        ctx.clearRect(0, 0, s, s);

        const cx = s / 2;
        const cy = s / 2;

        // Idle floating bounce
        const bounce = Math.sin(this.animTime * 2.5) * (4 * this.dpr);
        const headY = cy + bounce;

        // 1. Shadow underneath
        ctx.save();
        const shadowScale = 1 - (bounce / (20 * this.dpr));
        ctx.beginPath();
        ctx.ellipse(cx, s - 14 * this.dpr, 36 * this.dpr * shadowScale, 8 * this.dpr * shadowScale, 0, 0, Math.PI * 2);
        ctx.fillStyle = 'rgba(0, 0, 0, 0.12)';
        ctx.fill();
        ctx.restore();

        // 2. Antenna with pulsing LED
        ctx.save();
        ctx.beginPath();
        ctx.moveTo(cx, headY - 42 * this.dpr);
        ctx.lineTo(cx, headY - 56 * this.dpr);
        ctx.strokeStyle = this.theme.bodyColor;
        ctx.lineWidth = 3.5 * this.dpr;
        ctx.lineCap = 'round';
        ctx.stroke();

        // Antenna LED
        const ledPulse = Math.abs(Math.sin(this.animTime * 3));
        ctx.beginPath();
        ctx.arc(cx, headY - 58 * this.dpr, (5.5 + ledPulse * 1.5) * this.dpr, 0, Math.PI * 2);
        ctx.fillStyle = this.expression === 'thinking' ? '#10B981' : this.theme.antennaColor;
        ctx.shadowColor = ctx.fillStyle;
        ctx.shadowBlur = 10 * this.dpr;
        ctx.fill();
        ctx.restore();

        // 3. Main Head (Rounded organic capsule)
        ctx.save();
        const headW = 86 * this.dpr;
        const headH = 74 * this.dpr;
        const radius = 28 * this.dpr;

        // Gradient body
        const grad = ctx.createLinearGradient(cx - headW / 2, headY - headH / 2, cx + headW / 2, headY + headH / 2);
        grad.addColorStop(0, this.theme.bodyAccentColor);
        grad.addColorStop(1, this.theme.bodyColor);

        ctx.beginPath();
        ctx.roundRect(cx - headW / 2, headY - headH / 2, headW, headH, radius);
        ctx.fillStyle = grad;
        ctx.shadowColor = 'rgba(0, 0, 0, 0.2)';
        ctx.shadowBlur = 14 * this.dpr;
        ctx.shadowOffsetY = 6 * this.dpr;
        ctx.fill();
        ctx.restore();

        // 4. Cheeks (Blush)
        ctx.save();
        ctx.beginPath();
        ctx.arc(cx - 26 * this.dpr, headY + 10 * this.dpr, 7 * this.dpr, 0, Math.PI * 2);
        ctx.arc(cx + 26 * this.dpr, headY + 10 * this.dpr, 7 * this.dpr, 0, Math.PI * 2);
        ctx.fillStyle = this.theme.blushColor;
        ctx.fill();
        ctx.restore();

        // 5. Eyes & Pupils
        this.drawEyes(cx, headY);

        // 6. Mouth (Driven by LipSync & Expression)
        this.drawMouth(cx, headY);
    }

    private drawEyes(cx: number, cy: number): void {
        const ctx = this.ctx;
        const eyeOffset = 18 * this.dpr;
        const eyeY = cy - 4 * this.dpr;
        const eyeRadiusX = 11 * this.dpr;
        const eyeRadiusY = 14 * this.dpr * (1 - this.blinkProgress);

        if (this.blinkProgress >= 0.9) {
            // Closed eye arc
            ctx.save();
            ctx.strokeStyle = this.theme.pupilColor;
            ctx.lineWidth = 2.5 * this.dpr;
            ctx.lineCap = 'round';

            [-eyeOffset, eyeOffset].forEach(offsetX => {
                ctx.beginPath();
                ctx.arc(cx + offsetX, eyeY, 8 * this.dpr, 0.1 * Math.PI, 0.9 * Math.PI, false);
                ctx.stroke();
            });
            ctx.restore();
            return;
        }

        // Draw Left & Right Eyes
        [-eyeOffset, eyeOffset].forEach(offsetX => {
            const eyeCenterX = cx + offsetX;

            // Eye background
            ctx.save();
            ctx.beginPath();
            ctx.ellipse(eyeCenterX, eyeY, eyeRadiusX, Math.max(1, eyeRadiusY), 0, 0, Math.PI * 2);
            ctx.fillStyle = this.theme.eyeColor;
            ctx.fill();

            // Pupil with mouse gaze tracking
            const pupilMaxShiftX = 4.5 * this.dpr;
            const pupilMaxShiftY = 4.5 * this.dpr;
            const pupilX = eyeCenterX + this.lookX * pupilMaxShiftX;
            const pupilY = eyeY + this.lookY * pupilMaxShiftY;
            const pupilR = 6 * this.dpr * (1 - this.blinkProgress);

            ctx.beginPath();
            ctx.arc(pupilX, pupilY, Math.max(0.5, pupilR), 0, Math.PI * 2);
            ctx.fillStyle = this.theme.pupilColor;
            ctx.fill();

            // Pupil sparkle/highlight
            if (this.blinkProgress < 0.5) {
                ctx.beginPath();
                ctx.arc(pupilX - 2 * this.dpr, pupilY - 2 * this.dpr, 2 * this.dpr, 0, Math.PI * 2);
                ctx.fillStyle = 'rgba(255, 255, 255, 0.85)';
                ctx.fill();
            }
            ctx.restore();
        });
    }

    private drawMouth(cx: number, cy: number): void {
        const ctx = this.ctx;
        const mouthY = cy + 15 * this.dpr;
        const openness = this.voiceSync.getState().mouthOpen;

        ctx.save();
        ctx.fillStyle = this.theme.pupilColor;
        ctx.strokeStyle = this.theme.pupilColor;
        ctx.lineWidth = 2.5 * this.dpr;
        ctx.lineCap = 'round';

        if (this.isSpeaking && openness > 0.08) {
            // Lip-sync animated mouth shapes
            const mouthW = (10 + openness * 8) * this.dpr;
            const mouthH = (4 + openness * 12) * this.dpr;

            ctx.beginPath();
            ctx.ellipse(cx, mouthY, mouthW / 2, mouthH / 2, 0, 0, Math.PI * 2);
            ctx.fill();

            // Little tongue inside
            if (openness > 0.4) {
                ctx.beginPath();
                ctx.arc(cx, mouthY + mouthH / 4, 3.5 * this.dpr, 0, Math.PI);
                ctx.fillStyle = '#F472B6';
                ctx.fill();
            }
        } else {
            // Idle or smiling mouth
            const isHappy = this.expression === 'happy';
            ctx.beginPath();
            ctx.arc(cx, mouthY - 3 * this.dpr, isHappy ? 8 * this.dpr : 6 * this.dpr, 0.15 * Math.PI, 0.85 * Math.PI, false);
            ctx.stroke();
        }

        ctx.restore();
    }

    // Public API Methods

    public setExpression(expression: AssistantExpression): void {
        this.expression = expression;
    }

    public getExpression(): AssistantExpression {
        return this.expression;
    }

    public attachTTS(ttsInstance: { onboundary?: any; currentUtterance?: SpeechSynthesisUtterance } | any): void {
        if (ttsInstance?.currentUtterance) {
            this.voiceSync.attachUtterance(ttsInstance.currentUtterance);
        }
    }

    public attachUtterance(utterance: SpeechSynthesisUtterance): void {
        this.voiceSync.attachUtterance(utterance);
    }

    public destroy(): void {
        if (this.rafId) cancelAnimationFrame(this.rafId);
        if (this.nextBlinkTimer) clearTimeout(this.nextBlinkTimer);
        if (this.canvas && this.canvas.parentElement) {
            this.canvas.parentElement.removeChild(this.canvas);
        }
    }
}
