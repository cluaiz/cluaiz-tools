// BrowserSTT.ts
// Handles real-time speech transcription via Web Speech Recognition API

export class BrowserSTT {
    private recognition: any = null;
    private onResultCallback: ((text: string, isFinal: boolean) => void) | null = null;
    private onEndCallback: (() => void) | null = null;
    private onErrorCallback: ((error: string) => void) | null = null;
    private isListeningActive: boolean = false;

    constructor() {
        if (typeof window !== 'undefined') {
            const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;

            if (SpeechRecognition) {
                this.recognition = new SpeechRecognition();
                this.recognition.continuous = true;
                this.recognition.interimResults = true;
                this.recognition.lang = 'en-US';

                this.recognition.onresult = this.handleResult.bind(this);
                this.recognition.onerror = this.handleError.bind(this);
                this.recognition.onend = this.handleEnd.bind(this);
            } else {
                console.warn("[BrowserSTT] Speech Recognition API is not supported in this browser/webview.");
            }
        }
    }

    private handleResult(event: any) {
        if (!this.onResultCallback) return;

        let interimTranscript = '';
        let finalTranscript = '';

        for (let i = event.resultIndex; i < event.results.length; ++i) {
            if (event.results[i].isFinal) {
                finalTranscript += event.results[i][0].transcript;
            } else {
                interimTranscript += event.results[i][0].transcript;
            }
        }

        if (finalTranscript) {
            this.onResultCallback(finalTranscript.trim(), true);
        } else if (interimTranscript) {
            this.onResultCallback(interimTranscript.trim(), false);
        }
    }

    private handleError(event: any) {
        if (event.error !== 'no-speech') {
            console.error("[BrowserSTT] Recognition Error:", event.error);
        }
        if (this.onErrorCallback) this.onErrorCallback(event.error);
    }

    private handleEnd() {
        this.isListeningActive = false;
        if (this.onEndCallback) this.onEndCallback();
    }

    public start(
        onResult: (text: string, isFinal: boolean) => void,
        onError?: (error: string) => void,
        onEnd?: () => void,
        lang?: string
    ) {
        if (!this.recognition) {
            if (onError) onError("Speech Recognition not supported");
            return;
        }

        this.onResultCallback = onResult;
        this.onErrorCallback = onError || null;
        this.onEndCallback = onEnd || null;

        if (lang) {
            this.recognition.lang = lang;
        }

        try {
            this.recognition.start();
            this.isListeningActive = true;
        } catch (e) {
            console.warn("[BrowserSTT] Start call ignored, already running:", e);
        }
    }

    public stop() {
        if (this.recognition && this.isListeningActive) {
            this.recognition.stop();
            this.isListeningActive = false;
        }
    }

    public isSupported(): boolean {
        return !!this.recognition;
    }

    public isListening(): boolean {
        return this.isListeningActive;
    }
}
