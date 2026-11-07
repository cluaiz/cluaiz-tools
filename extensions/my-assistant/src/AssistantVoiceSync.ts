/**
 * AssistantVoiceSync.ts
 * Real-time speech synthesis boundary and audio event bridge for avatar lip-synchronization
 */

export type MouthShape = 'closed' | 'open' | 'wide' | 'round' | 'smile';

export interface VoiceSyncState {
    isSpeaking: boolean;
    mouthOpen: number; // 0.0 to 1.0
    mouthShape: MouthShape;
    audioLevel: number; // 0.0 to 1.0
}

export class AssistantVoiceSync {
    private isSpeaking = false;
    private mouthOpen = 0;
    private targetMouthOpen = 0;
    private mouthShape: MouthShape = 'closed';
    private currentUtterance: SpeechSynthesisUtterance | null = null;
    private boundaryTimer: number | null = null;
    private listeners: Array<(state: VoiceSyncState) => void> = [];

    constructor() {}

    /**
     * Subscribe to voice sync state updates
     */
    public onStateChange(listener: (state: VoiceSyncState) => void): () => void {
        this.listeners.push(listener);
        return () => {
            const index = this.listeners.indexOf(listener);
            if (index > -1) this.listeners.splice(index, 1);
        };
    }

    private notifyListeners(): void {
        const state: VoiceSyncState = {
            isSpeaking: this.isSpeaking,
            mouthOpen: this.mouthOpen,
            mouthShape: this.mouthShape,
            audioLevel: this.targetMouthOpen
        };
        for (const listener of this.listeners) {
            listener(state);
        }
    }

    /**
     * Attach a SpeechSynthesisUtterance instance to track boundaries
     */
    public attachUtterance(utterance: SpeechSynthesisUtterance): void {
        this.currentUtterance = utterance;

        utterance.addEventListener('start', () => {
            this.isSpeaking = true;
            this.targetMouthOpen = 0.5;
            this.mouthShape = 'open';
            this.notifyListeners();
        });

        utterance.addEventListener('boundary', (event: SpeechSynthesisEvent) => {
            if (event.name === 'word' || event.name === 'sentence') {
                // Word boundary triggers animated mouth pulses
                this.onWordBoundary();
            }
        });

        utterance.addEventListener('end', () => {
            this.reset();
        });

        utterance.addEventListener('error', () => {
            this.reset();
        });

        utterance.addEventListener('pause', () => {
            this.isSpeaking = false;
            this.targetMouthOpen = 0;
            this.mouthShape = 'closed';
            this.notifyListeners();
        });

        utterance.addEventListener('resume', () => {
            this.isSpeaking = true;
            this.targetMouthOpen = 0.6;
            this.mouthShape = 'open';
            this.notifyListeners();
        });
    }

    /**
     * Triggers dynamic mouth opening and viseme cycling upon word boundary
     */
    public onWordBoundary(): void {
        if (!this.isSpeaking) return;

        // Alternate mouth shapes for natural speech movement
        const shapes: MouthShape[] = ['open', 'wide', 'round'];
        this.mouthShape = shapes[Math.floor(Math.random() * shapes.length)];
        this.targetMouthOpen = 0.4 + Math.random() * 0.6;

        if (this.boundaryTimer) {
            clearTimeout(this.boundaryTimer);
        }

        // Return towards closed between words
        this.boundaryTimer = typeof window !== 'undefined' ? window.setTimeout(() => {
            if (this.isSpeaking) {
                this.targetMouthOpen = 0.15;
            }
        }, 120) : null;
    }

    /**
     * Updates interpolation per frame
     */
    public update(deltaTime: number): VoiceSyncState {
        if (this.isSpeaking) {
            // Smoothly lerp towards target mouth position
            const lerpSpeed = Math.min(1, deltaTime * 14);
            this.mouthOpen += (this.targetMouthOpen - this.mouthOpen) * lerpSpeed;
        } else {
            // Fast decay to closed
            this.mouthOpen = Math.max(0, this.mouthOpen - deltaTime * 8);
            if (this.mouthOpen <= 0.01) {
                this.mouthOpen = 0;
                this.mouthShape = 'closed';
            }
        }

        const state: VoiceSyncState = {
            isSpeaking: this.isSpeaking,
            mouthOpen: this.mouthOpen,
            mouthShape: this.mouthShape,
            audioLevel: this.mouthOpen
        };

        return state;
    }

    /**
     * Manually trigger simulated speech state
     */
    public setSimulatedSpeaking(speaking: boolean, intensity: number = 0.5): void {
        this.isSpeaking = speaking;
        if (speaking) {
            this.targetMouthOpen = intensity;
            this.mouthShape = 'open';
        } else {
            this.reset();
        }
    }

    /**
     * Reset speech state
     */
    public reset(): void {
        this.isSpeaking = false;
        this.mouthOpen = 0;
        this.targetMouthOpen = 0;
        this.mouthShape = 'closed';
        if (this.boundaryTimer) {
            clearTimeout(this.boundaryTimer);
            this.boundaryTimer = null;
        }
        this.notifyListeners();
    }

    public getState(): VoiceSyncState {
        return {
            isSpeaking: this.isSpeaking,
            mouthOpen: this.mouthOpen,
            mouthShape: this.mouthShape,
            audioLevel: this.mouthOpen
        };
    }
}
