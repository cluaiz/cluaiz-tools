// BrowserTTS.ts
// Handles Dual-Engine Text-to-Speech (Native OS + Microsoft Neural) with Live Karaoke Word Highlighting

import { detectLanguageFromText } from './UniversalVoiceRegistry';

export interface TTSConfig {
    pitch?: number;
    rate?: number;
    volume?: number;
    gender?: 'male' | 'female';
    enginePreference?: 'native' | 'microsoft-neural';
    voiceName?: string;
}

// Complete High-fidelity Microsoft Neural Voice Mappings (80+ locales with regional fallbacks)
export const MICROSOFT_NEURAL_VOICES: Record<string, string> = {
    'af-ZA': 'af-ZA-AdriNeural',
    'am-ET': 'am-ET-MekdesNeural',
    'ar-EG': 'ar-EG-SalmaNeural',
    'ar-SA': 'ar-SA-ZariyahNeural',
    'az-AZ': 'az-AZ-BanuNeural',
    'bg-BG': 'bg-BG-KalinaNeural',
    'bn-BD': 'bn-BD-NabanitaNeural',
    'bn-IN': 'bn-IN-TanishaaNeural',
    'ca-ES': 'ca-ES-JoanaNeural',
    'cs-CZ': 'cs-CZ-VlastaNeural',
    'cy-GB': 'cy-GB-NiaNeural',
    'da-DK': 'da-DK-ChristelNeural',
    'de-DE': 'de-DE-KatjaNeural',
    'el-GR': 'el-GR-AthinaNeural',
    'en-GB': 'en-GB-SoniaNeural',
    'en-US': 'en-US-AriaNeural',
    'en-IN': 'en-IN-NeerjaNeural',
    'es-ES': 'es-ES-ElviraNeural',
    'es-MX': 'es-MX-DaliaNeural',
    'et-EE': 'et-EE-AnuNeural',
    'fa-IR': 'fa-IR-DilaraNeural',
    'fi-FI': 'fi-FI-SelmaNeural',
    'fil-PH': 'fil-PH-BlessicaNeural',
    'fr-FR': 'fr-FR-DeniseNeural',
    'fr-CA': 'fr-CA-SylvieNeural',
    'gu-IN': 'gu-IN-DhwaniNeural',
    'he-IL': 'he-IL-HilaNeural',
    'hi-IN': 'hi-IN-SwaraNeural',
    'hr-HR': 'hr-HR-GabrijelaNeural',
    'hu-HU': 'hu-HU-NoemiNeural',
    'hy-AM': 'hy-AM-AnahitNeural',
    'id-ID': 'id-ID-GadisNeural',
    'is-IS': 'is-IS-GudrunNeural',
    'it-IT': 'it-IT-ElsaNeural',
    'ja-JP': 'ja-JP-NanamiNeural',
    'jv-ID': 'jv-ID-SitiNeural',
    'ka-GE': 'ka-GE-EkaNeural',
    'kk-KZ': 'kk-KZ-AigulNeural',
    'km-KH': 'km-KH-SreymomNeural',
    'kn-IN': 'kn-IN-SapnaNeural',
    'ko-KR': 'ko-KR-SunHiNeural',
    'lo-LA': 'lo-LA-KeomanyNeural',
    'lt-LT': 'lt-LT-OnaNeural',
    'lv-LV': 'lv-LV-EveritaNeural',
    'mk-MK': 'mk-MK-MarijaNeural',
    'ml-IN': 'ml-IN-SobhanaNeural',
    'mn-MN': 'mn-MN-YesuiNeural',
    'mr-IN': 'mr-IN-AarohiNeural',
    'ms-MY': 'ms-MY-YasminNeural',
    'my-MM': 'my-MM-NilarNeural',
    'ne-NP': 'ne-NP-HemkalaNeural',
    'nl-NL': 'nl-NL-ColetteNeural',
    'nb-NO': 'nb-NO-PernilleNeural',
    'pa-IN': 'pa-IN-OjasNeural',
    'pl-PL': 'pl-PL-ZofiaNeural',
    'pt-BR': 'pt-BR-FranciscaNeural',
    'pt-PT': 'pt-PT-RaquelNeural',
    'ro-RO': 'ro-RO-AlinaNeural',
    'ru-RU': 'ru-RU-SvetlanaNeural',
    'si-LK': 'si-LK-ThiliniNeural',
    'sk-SK': 'sk-SK-ViktoriaNeural',
    'sl-SI': 'sl-SI-PetraNeural',
    'sq-AL': 'sq-AL-AnilaNeural',
    'sr-RS': 'sr-RS-SophieNeural',
    'su-ID': 'su-ID-TutiNeural',
    'sv-SE': 'sv-SE-SofieNeural',
    'sw-KE': 'sw-KE-ZuriNeural',
    'ta-IN': 'ta-IN-PallaviNeural',
    'te-IN': 'te-IN-ShrutiNeural',
    'th-TH': 'th-TH-PremwadeeNeural',
    'tr-TR': 'tr-TR-EmelNeural',
    'uk-UA': 'uk-UA-PolinaNeural',
    'ur-PK': 'ur-PK-UzmaNeural',
    'ur-IN': 'ur-IN-GulNeural',
    'uz-UZ': 'uz-UZ-MadinaNeural',
    'vi-VN': 'vi-VN-HoaiMyNeural',
    'zh-CN': 'zh-CN-XiaoxiaoNeural',
    'zh-HK': 'zh-HK-HiuGaaiNeural',
    'zh-TW': 'zh-TW-HsiaoChenNeural',
    'zu-ZA': 'zu-ZA-ThandoNeural',

    // Regional Dialect Fallbacks
    'as-IN': 'en-IN-NeerjaNeural',
    'be-BY': 'ru-RU-SvetlanaNeural',
    'bs-BA': 'sr-RS-SophieNeural',
    'ga-IE': 'en-GB-SoniaNeural',
    'gl-ES': 'es-ES-ElviraNeural',
    'mt-MT': 'en-GB-SoniaNeural',
    'ps-AF': 'ar-SA-ZariyahNeural',
    'so-SO': 'en-GB-SoniaNeural',
    'sw-TZ': 'en-GB-SoniaNeural',
    'ks-IN': 'ur-PK-UzmaNeural',
    'sd-IN': 'ur-PK-UzmaNeural',
    'bho-IN': 'hi-IN-SwaraNeural',
    'mai-IN': 'hi-IN-SwaraNeural',
    'doi-IN': 'hi-IN-SwaraNeural',
    'kok-IN': 'mr-IN-AarohiNeural',
    'or-IN': 'bn-IN-TanishaaNeural',
};

export class BrowserTTS {
    private static currentUtterance: SpeechSynthesisUtterance | null = null;
    private static isSpeakingNow: boolean = false;
    private static isPausedNow: boolean = false;

    /**
     * Retrieves all voices currently loaded in the host browser / webview.
     */
    public static getVoices(): Promise<SpeechSynthesisVoice[]> {
        return new Promise((resolve) => {
            if (typeof window === 'undefined' || !('speechSynthesis' in window)) {
                resolve([]);
                return;
            }

            let voices = window.speechSynthesis.getVoices();
            if (voices.length > 0) {
                resolve(voices);
                return;
            }

            const handler = () => {
                voices = window.speechSynthesis.getVoices();
                window.speechSynthesis.removeEventListener('voiceschanged', handler);
                resolve(voices);
            };

            window.speechSynthesis.addEventListener('voiceschanged', handler);
            setTimeout(() => {
                window.speechSynthesis.removeEventListener('voiceschanged', handler);
                resolve(window.speechSynthesis.getVoices());
            }, 600);
        });
    }

    /**
     * Speaks the provided text with boundary tracking for live word highlighting.
     */
    public static async speak(
        text: string,
        config?: TTSConfig & { lang?: string },
        onStart?: () => void,
        onEnd?: () => void,
        onBoundary?: (charIndex: number, length?: number) => void,
        onError?: (err: any) => void
    ) {
        if (typeof window === 'undefined' || !('speechSynthesis' in window)) {
            if (onError) onError(new Error("Speech Synthesis not available in this environment."));
            return;
        }

        // Cancel previous speech if still active
        this.stop();

        const detectedLang = config?.lang || detectLanguageFromText(text);
        const utterance = new SpeechSynthesisUtterance(text);
        utterance.lang = detectedLang;
        utterance.rate = config?.rate || 1.0;
        utterance.pitch = config?.pitch || 1.0;
        utterance.volume = config?.volume || 1.0;

        // Resolve suitable voice (Microsoft Neural vs Native)
        const availableVoices = await this.getVoices();
        let selectedVoice: SpeechSynthesisVoice | undefined;

        if (config?.voiceName) {
            selectedVoice = availableVoices.find(v => v.name === config.voiceName);
        }

        if (!selectedVoice) {
            const neuralName = MICROSOFT_NEURAL_VOICES[detectedLang] || MICROSOFT_NEURAL_VOICES[detectedLang.split('-')[0]];
            if (config?.enginePreference !== 'native' && neuralName) {
                const keyword = neuralName.replace('Neural', '').toLowerCase();
                selectedVoice = availableVoices.find(v => 
                    v.name.toLowerCase().includes(keyword) || 
                    (v.name.includes("Natural") && v.lang.startsWith(detectedLang.split('-')[0]))
                );
            }
        }

        if (!selectedVoice) {
            // Fallback to closest language match
            selectedVoice = availableVoices.find(v => v.lang.replace('_', '-') === detectedLang)
                || availableVoices.find(v => v.lang.startsWith(detectedLang.split('-')[0]))
                || availableVoices.find(v => v.default);
        }

        if (selectedVoice) {
            utterance.voice = selectedVoice;
        }

        // Attach Event Listeners
        utterance.onstart = () => {
            this.isSpeakingNow = true;
            this.isPausedNow = false;
            if (onStart) onStart();
        };

        utterance.onend = () => {
            this.isSpeakingNow = false;
            this.isPausedNow = false;
            this.currentUtterance = null;
            if (onEnd) onEnd();
        };

        utterance.onerror = (e) => {
            this.isSpeakingNow = false;
            this.isPausedNow = false;
            this.currentUtterance = null;
            if (onError) onError(e);
        };

        // Live Karaoke Word Highlighting Hook
        utterance.onboundary = (event: SpeechSynthesisEvent) => {
            if (onBoundary && (event.name === 'word' || event.charIndex !== undefined)) {
                onBoundary(event.charIndex, event.charLength || 0);
            }
        };

        this.currentUtterance = utterance;
        window.speechSynthesis.speak(utterance);
    }

    public static pause() {
        if (typeof window !== 'undefined' && 'speechSynthesis' in window && this.isSpeakingNow) {
            window.speechSynthesis.pause();
            this.isPausedNow = true;
        }
    }

    public static resume() {
        if (typeof window !== 'undefined' && 'speechSynthesis' in window && this.isPausedNow) {
            window.speechSynthesis.resume();
            this.isPausedNow = false;
        }
    }

    public static stop() {
        if (typeof window !== 'undefined' && 'speechSynthesis' in window) {
            window.speechSynthesis.cancel();
            this.isSpeakingNow = false;
            this.isPausedNow = false;
            this.currentUtterance = null;
        }
    }

    public static isSpeaking(): boolean {
        return this.isSpeakingNow;
    }

    public static isPaused(): boolean {
        return this.isPausedNow;
    }
}
