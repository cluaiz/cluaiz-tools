/**
 * UniversalVoiceRegistry.ts
 * A comprehensive mapping of ISO Language/Region codes to Flags, Names, and Detection Regex.
 * This allows the system to support 150+ languages natively with ZERO maintenance.
 */

export interface VoiceMetadata {
    code: string;       // BCP-47 / ISO code (e.g., 'en-US', 'hi-IN')
    name: string;       // Native Name
    displayName: string; // English Name
    flag: string;       // Emoji Flag
    regex?: RegExp;     // Script detection regex
}

export const UNIVERSAL_VOICE_REGISTRY: Record<string, VoiceMetadata> = {
    // A-Z Order - Direct Regex Literals inside each object (No separate constants)
    "af-ZA": { code: "af-ZA", name: "Afrikaans", displayName: "Afrikaans", flag: "🇿🇦", regex: /[a-zA-Z]/i },
    "am-ET": { code: "am-ET", name: "አማርኛ", displayName: "Amharic", flag: "🇪🇹", regex: /[\u1200-\u137F]/ },
    "ar-AE": { code: "ar-AE", name: "العربية", displayName: "Arabic (UAE)", flag: "🇦🇪", regex: /[\u0600-\u06FF]/ },
    "ar-BH": { code: "ar-BH", name: "العربية", displayName: "Arabic (Bahrain)", flag: "🇧🇭", regex: /[\u0600-\u06FF]/ },
    "ar-DZ": { code: "ar-DZ", name: "العربية", displayName: "Arabic (Algeria)", flag: "🇩🇿", regex: /[\u0600-\u06FF]/ },
    "ar-EG": { code: "ar-EG", name: "العربية", displayName: "Arabic (Egypt)", flag: "🇪🇬", regex: /[\u0600-\u06FF]/ },
    "ar-IQ": { code: "ar-IQ", name: "العربية", displayName: "Arabic (Iraq)", flag: "🇮🇶", regex: /[\u0600-\u06FF]/ },
    "ar-JO": { code: "ar-JO", name: "العربية", displayName: "Arabic (Jordan)", flag: "🇯🇴", regex: /[\u0600-\u06FF]/ },
    "ar-KW": { code: "ar-KW", name: "العربية", displayName: "Arabic (Kuwait)", flag: "🇰🇼", regex: /[\u0600-\u06FF]/ },
    "ar-LB": { code: "ar-LB", name: "العربية", displayName: "Arabic (Lebanon)", flag: "🇱🇧", regex: /[\u0600-\u06FF]/ },
    "ar-LY": { code: "ar-LY", name: "العربية", displayName: "Arabic (Libya)", flag: "🇱🇾", regex: /[\u0600-\u06FF]/ },
    "ar-MA": { code: "ar-MA", name: "العربية", displayName: "Arabic (Morocco)", flag: "🇲🇦", regex: /[\u0600-\u06FF]/ },
    "ar-OM": { code: "ar-OM", name: "العربية", displayName: "Arabic (Oman)", flag: "🇴🇲", regex: /[\u0600-\u06FF]/ },
    "ar-QA": { code: "ar-QA", name: "العربية", displayName: "Arabic (Qatar)", flag: "🇶🇦", regex: /[\u0600-\u06FF]/ },
    "ar-SA": { code: "ar-SA", name: "العربية", displayName: "Arabic (Saudi Arabia)", flag: "🇸🇦", regex: /[\u0600-\u06FF]/ },
    "ar-SY": { code: "ar-SY", name: "العربية", displayName: "Arabic (Syria)", flag: "🇸🇾", regex: /[\u0600-\u06FF]/ },
    "ar-TN": { code: "ar-TN", name: "العربية", displayName: "Arabic (Tunisia)", flag: "🇹🇳", regex: /[\u0600-\u06FF]/ },
    "ar-YE": { code: "ar-YE", name: "العربية", displayName: "Arabic (Yemen)", flag: "🇾🇪", regex: /[\u0600-\u06FF]/ },
    "as-IN": { code: "as-IN", name: "অসমীয়া", displayName: "Assamese", flag: "🇮🇳", regex: /[\u0980-\u09FF]/ },
    "az-AZ": { code: "az-AZ", name: "Azərbaycanca", displayName: "Azerbaijani", flag: "🇦🇿", regex: /[a-zA-ZəışçğöüƏIŞÇĞÖÜ]/i },
    "be-BY": { code: "be-BY", name: "Беларуская", displayName: "Belarusian", flag: "🇧🇾", regex: /[\u0400-\u04FF]/ },
    "bg-BG": { code: "bg-BG", name: "Български", displayName: "Bulgarian", flag: "🇧🇬", regex: /[\u0400-\u04FF]/ },
    "bn-BD": { code: "bn-BD", name: "বাংলা", displayName: "Bengali (BD)", flag: "🇧🇩", regex: /[\u0980-\u09FF]/ },
    "bn-IN": { code: "bn-IN", name: "বাংলা", displayName: "Bengali (IN)", flag: "🇮🇳", regex: /[\u0980-\u09FF]/ },
    "bs-BA": { code: "bs-BA", name: "Bosanski", displayName: "Bosnian", flag: "🇧🇦", regex: /[a-zA-Z]/i },
    "ca-ES": { code: "ca-ES", name: "Català", displayName: "Catalan", flag: "🇪🇸", regex: /[a-zA-Z]/i },
    "cs-CZ": { code: "cs-CZ", name: "Čeština", displayName: "Czech", flag: "🇨🇿", regex: /[a-zA-Z]/i },
    "cy-GB": { code: "cy-GB", name: "Cymraeg", displayName: "Welsh", flag: "🇬🇧", regex: /[a-zA-Z]/i },
    "da-DK": { code: "da-DK", name: "Dansk", displayName: "Danish", flag: "🇩🇰", regex: /[a-zA-Z]/i },
    "de-AT": { code: "de-AT", name: "Deutsch", displayName: "German (AT)", flag: "🇦🇹", regex: /[a-zA-Z]/i },
    "de-CH": { code: "de-CH", name: "Deutsch", displayName: "German (CH)", flag: "🇨🇭", regex: /[a-zA-Z]/i },
    "de-DE": { code: "de-DE", name: "Deutsch", displayName: "German (DE)", flag: "🇩🇪", regex: /[a-zA-Z]/i },
    "el-GR": { code: "el-GR", name: "Ελληνικά", displayName: "Greek", flag: "🇬🇷", regex: /[\u0370-\u03FF]/ },
    "en-AU": { code: "en-AU", name: "English", displayName: "English (AU)", flag: "🇦🇺", regex: /[a-zA-Z]/i },
    "en-CA": { code: "en-CA", name: "English", displayName: "English (CA)", flag: "🇨🇦", regex: /[a-zA-Z]/i },
    "en-GB": { code: "en-GB", name: "English", displayName: "English (UK)", flag: "🇬🇧", regex: /[a-zA-Z]/i },
    "en-GH": { code: "en-GH", name: "English", displayName: "English (GH)", flag: "🇬🇭", regex: /[a-zA-Z]/i },
    "en-HK": { code: "en-HK", name: "English", displayName: "English (HK)", flag: "🇭🇰", regex: /[a-zA-Z]/i },
    "en-IE": { code: "en-IE", name: "English", displayName: "English (IE)", flag: "🇮🇪", regex: /[a-zA-Z]/i },
    "en-IN": { code: "en-IN", name: "English", displayName: "English (IN)", flag: "🇮🇳", regex: /[a-zA-Z]/i },
    "en-KE": { code: "en-KE", name: "English", displayName: "English (KE)", flag: "🇰🇪", regex: /[a-zA-Z]/i },
    "en-NG": { code: "en-NG", name: "English", displayName: "English (NG)", flag: "🇳🇬", regex: /[a-zA-Z]/i },
    "en-NZ": { code: "en-NZ", name: "English", displayName: "English (NZ)", flag: "🇳🇿", regex: /[a-zA-Z]/i },
    "en-PH": { code: "en-PH", name: "English", displayName: "English (PH)", flag: "🇵🇭", regex: /[a-zA-Z]/i },
    "en-SG": { code: "en-SG", name: "English", displayName: "English (SG)", flag: "🇸🇬", regex: /[a-zA-Z]/i },
    "en-TZ": { code: "en-TZ", name: "English", displayName: "English (TZ)", flag: "🇹🇿", regex: /[a-zA-Z]/i },
    "en-US": { code: "en-US", name: "English", displayName: "English (US)", flag: "🇺🇸", regex: /[a-zA-Z]/i },
    "en-ZA": { code: "en-ZA", name: "English", displayName: "English (ZA)", flag: "🇿🇦", regex: /[a-zA-Z]/i },
    "es-AR": { code: "es-AR", name: "Español", displayName: "Spanish (AR)", flag: "🇦🇷", regex: /[a-zA-Z]/i },
    "es-BO": { code: "es-BO", name: "Español", displayName: "Spanish (BO)", flag: "🇧🇴", regex: /[a-zA-Z]/i },
    "es-CL": { code: "es-CL", name: "Español", displayName: "Spanish (CL)", flag: "🇨🇱", regex: /[a-zA-Z]/i },
    "es-CO": { code: "es-CO", name: "Español", displayName: "Spanish (CO)", flag: "🇨🇴", regex: /[a-zA-Z]/i },
    "es-CR": { code: "es-CR", name: "Español", displayName: "Spanish (CR)", flag: "🇨🇷", regex: /[a-zA-Z]/i },
    "es-CU": { code: "es-CU", name: "Español", displayName: "Spanish (CU)", flag: "🇨🇺", regex: /[a-zA-Z]/i },
    "es-DO": { code: "es-DO", name: "Español", displayName: "Spanish (DO)", flag: "🇩🇴", regex: /[a-zA-Z]/i },
    "es-EC": { code: "es-EC", name: "Español", displayName: "Spanish (EC)", flag: "🇪🇨", regex: /[a-zA-Z]/i },
    "es-ES": { code: "es-ES", name: "Español", displayName: "Spanish (ES)", flag: "🇪🇸", regex: /[a-zA-Záéíóúüñ¿¡ÁÉÍÓÚÜÑ]/i },
    "es-GT": { code: "es-GT", name: "Español", displayName: "Spanish (GT)", flag: "🇬🇹", regex: /[a-zA-Z]/i },
    "es-HN": { code: "es-HN", name: "Español", displayName: "Spanish (HN)", flag: "🇭🇳", regex: /[a-zA-Z]/i },
    "es-MX": { code: "es-MX", name: "Español", displayName: "Spanish (MX)", flag: "🇲🇽", regex: /[a-zA-Z]/i },
    "es-NI": { code: "es-NI", name: "Español", displayName: "Spanish (NI)", flag: "🇳🇮", regex: /[a-zA-Z]/i },
    "es-PA": { code: "es-PA", name: "Español", displayName: "Spanish (PA)", flag: "🇵🇦", regex: /[a-zA-Z]/i },
    "es-PE": { code: "es-PE", name: "Español", displayName: "Spanish (PE)", flag: "🇵🇪", regex: /[a-zA-Z]/i },
    "es-PR": { code: "es-PR", name: "Español", displayName: "Spanish (PR)", flag: "🇵🇷", regex: /[a-zA-Z]/i },
    "es-PY": { code: "es-PY", name: "Español", displayName: "Spanish (PY)", flag: "🇵🇾", regex: /[a-zA-Z]/i },
    "es-SV": { code: "es-SV", name: "Español", displayName: "Spanish (SV)", flag: "🇸🇻", regex: /[a-zA-Z]/i },
    "es-US": { code: "es-US", name: "Español", displayName: "Spanish (US)", flag: "🇺🇸", regex: /[a-zA-Z]/i },
    "es-UY": { code: "es-UY", name: "Español", displayName: "Spanish (UY)", flag: "🇺🇾", regex: /[a-zA-Z]/i },
    "es-VE": { code: "es-VE", name: "Español", displayName: "Spanish (VE)", flag: "🇻🇪", regex: /[a-zA-Z]/i },
    "et-EE": { code: "et-EE", name: "Eesti", displayName: "Estonian", flag: "🇪🇪", regex: /[a-zA-Z]/i },
    "fa-IR": { code: "fa-IR", name: "فارسی", displayName: "Persian", flag: "🇮🇷", regex: /[\u0600-\u06FF]/ },
    "fi-FI": { code: "fi-FI", name: "Suomi", displayName: "Finnish", flag: "🇫🇮", regex: /[a-zA-Z]/i },
    "fil-PH": { code: "fil-PH", name: "Filipino", displayName: "Filipino", flag: "🇵🇭", regex: /[a-zA-Z]/i },
    "fr-BE": { code: "fr-BE", name: "Français", displayName: "French (BE)", flag: "🇧🇪", regex: /[a-zA-Z]/i },
    "fr-CA": { code: "fr-CA", name: "Français", displayName: "French (CA)", flag: "🇨🇦", regex: /[a-zA-Z]/i },
    "fr-CH": { code: "fr-CH", name: "Français", displayName: "French (CH)", flag: "🇨🇭", regex: /[a-zA-Z]/i },
    "fr-FR": { code: "fr-FR", name: "Français", displayName: "French (FR)", flag: "🇫🇷", regex: /[a-zA-ZàâçéèêëîïôûùÿÀÂÇÉÈÊËÎÏÔÛÙŸ]/i },
    "ga-IE": { code: "ga-IE", name: "Gaeilge", displayName: "Irish", flag: "🇮🇪", regex: /[a-zA-Z]/i },
    "gl-ES": { code: "gl-ES", name: "Galego", displayName: "Galician", flag: "🇪🇸", regex: /[a-zA-Z]/i },
    "gu-IN": { code: "gu-IN", name: "ગુજરાતી", displayName: "Gujarati", flag: "🇮🇳", regex: /[\u0A80-\u0AFF]/ },
    "he-IL": { code: "he-IL", name: "עברית", displayName: "Hebrew", flag: "🇮🇱", regex: /[\u0590-\u05FF]/ },
    "hi-IN": { code: "hi-IN", name: "हिन्दी", displayName: "Hindi", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
    "hr-HR": { code: "hr-HR", name: "Hrvatski", displayName: "Croatian", flag: "🇭🇷", regex: /[a-zA-Z]/i },
    "hu-HU": { code: "hu-HU", name: "Magyar", displayName: "Hungarian", flag: "🇭🇺", regex: /[a-zA-Z]/i },
    "hy-AM": { code: "hy-AM", name: "Հայերեն", displayName: "Armenian", flag: "🇦🇲", regex: /[\u0530-\u058F]/ },
    "id-ID": { code: "id-ID", name: "Bahasa Indonesia", displayName: "Indonesian", flag: "🇮🇩", regex: /[a-zA-Z]/i },
    "is-IS": { code: "is-IS", name: "Íslenska", displayName: "Icelandic", flag: "🇮🇸", regex: /[a-zA-Z]/i },
    "it-CH": { code: "it-CH", name: "Italiano", displayName: "Italian (CH)", flag: "🇨🇭", regex: /[a-zA-Z]/i },
    "it-IT": { code: "it-IT", name: "Italiano", displayName: "Italian (IT)", flag: "🇮🇹", regex: /[a-zA-Zàèéìòù]/i },
    "ja-JP": { code: "ja-JP", name: "日本語", displayName: "Japanese", flag: "🇯🇵", regex: /[\u3040-\u309F\u30A0-\u30FF\u4E00-\u9FAF]/ },
    "jv-ID": { code: "jv-ID", name: "Basa Jawa", displayName: "Javanese", flag: "🇮🇩", regex: /[a-zA-Z]/i },
    "ka-GE": { code: "ka-GE", name: "ქართული", displayName: "Georgian", flag: "🇬🇪", regex: /[\u10A0-\u10FF]/ },
    "kk-KZ": { code: "kk-KZ", name: "Қазақ тілі", displayName: "Kazakh", flag: "🇰🇿", regex: /[\u0400-\u04FF]/ },
    "km-KH": { code: "km-KH", name: "ខ្មែរ", displayName: "Khmer", flag: "🇰🇭", regex: /[\u1780-\u17FF]/ },
    "kn-IN": { code: "kn-IN", name: "ಕನ್ನಡ", displayName: "Kannada", flag: "🇮🇳", regex: /[\u0C80-\u0CFF]/ },
    "ko-KR": { code: "ko-KR", name: "한국어", displayName: "Korean", flag: "🇰🇷", regex: /[\uAC00-\uD7AF]/ },
    "lo-LA": { code: "lo-LA", name: "ລາວ", displayName: "Lao", flag: "🇱🇦", regex: /[\u0E80-\u0EFF]/ },
    "lt-LT": { code: "lt-LT", name: "Lietuvių", displayName: "Lithuanian", flag: "🇱🇹", regex: /[a-zA-Z]/i },
    "lv-LV": { code: "lv-LV", name: "Latviešu", displayName: "Latvian", flag: "🇱🇻", regex: /[a-zA-Z]/i },
    "mk-MK": { code: "mk-MK", name: "Македонски", displayName: "Macedonian", flag: "🇲🇰", regex: /[\u0400-\u04FF]/ },
    "ml-IN": { code: "ml-IN", name: "മലയാളം", displayName: "Malayalam", flag: "🇮🇳", regex: /[\u0D00-\u0D7F]/ },
    "mn-MN": { code: "mn-MN", name: "Монгол", displayName: "Mongolian", flag: "🇲🇳", regex: /[\u0400-\u04FF]/ },
    "mr-IN": { code: "mr-IN", name: "मराठी", displayName: "Marathi", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
    "ms-MY": { code: "ms-MY", name: "Bahasa Melayu", displayName: "Malay", flag: "🇲🇾", regex: /[a-zA-Z]/i },
    "mt-MT": { code: "mt-MT", name: "Malti", displayName: "Maltese", flag: "🇲🇹", regex: /[a-zA-Z]/i },
    "my-MM": { code: "my-MM", name: "မြန်မာ", displayName: "Burmese", flag: "🇲🇲", regex: /[\u1000-\u109F]/ },
    "nb-NO": { code: "nb-NO", name: "Norsk bokmål", displayName: "Norwegian", flag: "🇳🇴", regex: /[a-zA-Z]/i },
    "ne-NP": { code: "ne-NP", name: "नेपाली", displayName: "Nepali", flag: "🇳🇵", regex: /[\u0900-\u097F]/ },
    "nl-BE": { code: "nl-BE", name: "Nederlands", displayName: "Dutch (BE)", flag: "🇧🇪", regex: /[a-zA-Z]/i },
    "nl-NL": { code: "nl-NL", name: "Nederlands", displayName: "Dutch (NL)", flag: "🇳🇱", regex: /[a-zA-Z]/i },
    "pa-IN": { code: "pa-IN", name: "ਪੰਜਾਬੀ", displayName: "Punjabi", flag: "🇮🇳", regex: /[\u0A00-\u0A7F]/ },
    "pl-PL": { code: "pl-PL", name: "Polski", displayName: "Polish", flag: "🇵🇱", regex: /[a-zA-Z]/i },
    "ps-AF": { code: "ps-AF", name: "پښتو", displayName: "Pashto", flag: "🇦🇫", regex: /[\u0600-\u06FF]/ },
    "pt-BR": { code: "pt-BR", name: "Português", displayName: "Portuguese (BR)", flag: "🇧🇷", regex: /[a-zA-Zãõ]/i },
    "pt-PT": { code: "pt-PT", name: "Português", displayName: "Portuguese (PT)", flag: "🇵🇹", regex: /[a-zA-Zãõ]/i },
    "ro-RO": { code: "ro-RO", name: "Română", displayName: "Romanian", flag: "🇷🇴", regex: /[a-zA-Z]/i },
    "ru-RU": { code: "ru-RU", name: "Русский", displayName: "Russian", flag: "🇷🇺", regex: /[\u0400-\u04FF]/ },
    "si-LK": { code: "si-LK", name: "සිංහල", displayName: "Sinhala", flag: "🇱🇰", regex: /[\u0D80-\u0DFF]/ },
    "sk-SK": { code: "sk-SK", name: "Slovenčina", displayName: "Slovak", flag: "🇸🇰", regex: /[a-zA-Z]/i },
    "sl-SI": { code: "sl-SI", name: "Slovenščina", displayName: "Slovenian", flag: "🇸🇮", regex: /[a-zA-Z]/i },
    "so-SO": { code: "so-SO", name: "Soomaali", displayName: "Somali", flag: "🇸🇴", regex: /[a-zA-Z]/i },
    "sq-AL": { code: "sq-AL", name: "Shqip", displayName: "Albanian", flag: "🇦🇱", regex: /[a-zA-Z]/i },
    "sr-RS": { code: "sr-RS", name: "Српски", displayName: "Serbian", flag: "🇷🇸", regex: /[\u0400-\u04FF]/ },
    "su-ID": { code: "su-ID", name: "Basa Sunda", displayName: "Sundanese", flag: "🇮🇩", regex: /[a-zA-Z]/i },
    "sv-SE": { code: "sv-SE", name: "Svenska", displayName: "Swedish", flag: "🇸🇪", regex: /[a-zA-Z]/i },
    "sw-KE": { code: "sw-KE", name: "Kiswahili", displayName: "Swahili (KE)", flag: "🇰🇪", regex: /[a-zA-Z]/i },
    "sw-TZ": { code: "sw-TZ", name: "Kiswahili", displayName: "Swahili (TZ)", flag: "🇹🇿", regex: /[a-zA-Z]/i },
    "ta-IN": { code: "ta-IN", name: "தமிழ்", displayName: "Tamil (IN)", flag: "🇮🇳", regex: /[\u0B80-\u0BFF]/ },
    "ta-LK": { code: "ta-LK", name: "தமிழ்", displayName: "Tamil (LK)", flag: "🇱🇰", regex: /[\u0B80-\u0BFF]/ },
    "ta-MY": { code: "ta-MY", name: "தமிழ்", displayName: "Tamil (MY)", flag: "🇲🇾", regex: /[\u0B80-\u0BFF]/ },
    "ta-SG": { code: "ta-SG", name: "தமிழ்", displayName: "Tamil (SG)", flag: "🇸🇬", regex: /[\u0B80-\u0BFF]/ },
    "te-IN": { code: "te-IN", name: "తెలుగు", displayName: "Telugu", flag: "🇮🇳", regex: /[\u0C00-\u0C7F]/ },
    "th-TH": { code: "th-TH", name: "ไทย", displayName: "Thai", flag: "🇹🇭", regex: /[\u0E00-\u0E7F]/ },
    "tr-TR": { code: "tr-TR", name: "Türkçe", displayName: "Turkish", flag: "🇹🇷", regex: /[a-zA-ZğüşöçİĞÜŞÖÇı]/i },
    "uk-UA": { code: "uk-UA", name: "Українська", displayName: "Ukrainian", flag: "🇺🇦", regex: /[\u0400-\u04FF]/ },
    "ur-IN": { code: "ur-IN", name: "اردو", displayName: "Urdu (IN)", flag: "🇮🇳", regex: /[\u0600-\u06FF]/ },
    "ur-PK": { code: "ur-PK", name: "اردو", displayName: "Urdu (PK)", flag: "🇵🇰", regex: /[\u0600-\u06FF]/ },
    "uz-UZ": { code: "uz-UZ", name: "O'zbekcha", displayName: "Uzbek", flag: "🇺🇿", regex: /[a-zA-Z]/i },
    "vi-VN": { code: "vi-VN", name: "Tiếng Việt", displayName: "Vietnamese", flag: "🇻🇳", regex: /[a-zA-Z]/i },
    "zh-CN": { code: "zh-CN", name: "中文", displayName: "Chinese (CN)", flag: "🇨🇳", regex: /[\u4E00-\u9FFF]/ },
    "zh-HK": { code: "zh-HK", name: "中文", displayName: "Chinese (HK)", flag: "🇭🇰", regex: /[\u4E00-\u9FFF]/ },
    "zh-TW": { code: "zh-TW", name: "中文", displayName: "Chinese (TW)", flag: "🇹🇼", regex: /[\u4E00-\u9FFF]/ },
    "zu-ZA": { code: "zu-ZA", name: "isiZulu", displayName: "Zulu", flag: "🇿🇦", regex: /[a-zA-Z]/i },
    "yo-NG": { code: "yo-NG", name: "Yorùbá", displayName: "Yoruba", flag: "🇳🇬", regex: /[a-zA-Z]/i },
    "ha-NE": { code: "ha-NE", name: "Hausa", displayName: "Hausa", flag: "🇳🇪", regex: /[a-zA-Z]/i },
    "ig-NG": { code: "ig-NG", name: "Igbo", displayName: "Igbo", flag: "🇳🇬", regex: /[a-zA-Z]/i },
    "or-IN": { code: "or-IN", name: "ଓଡ଼ିଆ", displayName: "Odia", flag: "🇮🇳", regex: /[\u0B00-\u0B7F]/ },
    "ks-IN": { code: "ks-IN", name: "کٲشُر", displayName: "Kashmiri", flag: "🇮🇳", regex: /[\u0600-\u06FF]/ },
    "sd-IN": { code: "sd-IN", name: "سنڌي", displayName: "Sindhi", flag: "🇮🇳", regex: /[\u0600-\u06FF]/ },
    "mai-IN": { code: "mai-IN", name: "मैथिली", displayName: "Maithili", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
    "sat-IN": { code: "sat-IN", name: "ᱥᱟᱱᱛᱟᱲᱤ", displayName: "Santali", flag: "🇮🇳", regex: /[\u1C50-\u1C7F]/ },
    "mni-IN": { code: "mni-IN", name: "ꯃꯤꯇꯩ ꯂꯣꯟ", displayName: "Manipuri", flag: "🇮🇳", regex: /[\uABC0-\uABFF]/ },
    "kok-IN": { code: "kok-IN", name: "कोंकणी", displayName: "Konkani", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
    "doi-IN": { code: "doi-IN", name: "डोगरी", displayName: "Dogri", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
    "brx-IN": { code: "brx-IN", name: "बड़ो", displayName: "Bodo", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
    "bho-IN": { code: "bho-IN", name: "भोजपुरी", displayName: "Bhojpuri", flag: "🇮🇳", regex: /[\u0900-\u097F]/ },
};

/**
 * Helper to get flag and name by any code (e.g. 'en-US' or just 'en')
 */
export const getVoiceMetadata = (code: string): VoiceMetadata => {
    if (UNIVERSAL_VOICE_REGISTRY[code]) return UNIVERSAL_VOICE_REGISTRY[code];
    const normalized = code.replace('_', '-');
    if (UNIVERSAL_VOICE_REGISTRY[normalized]) return UNIVERSAL_VOICE_REGISTRY[normalized];
    const langOnly = normalized.split('-')[0];
    const fallback = Object.values(UNIVERSAL_VOICE_REGISTRY).find(v => v.code.startsWith(langOnly));
    return fallback || { code, name: "Unknown", displayName: `Unknown (${code})`, flag: "🌐" };
};

/**
 * Detect language from text using the registry's regex patterns.
 * Centralized logic - one place to update for all files.
 */
export const detectLanguageFromText = (text: string): string => {
    if (!text || text.trim().length === 0) return 'en-US';

    const entries = Object.values(UNIVERSAL_VOICE_REGISTRY);

    // Priority 1: Specific scripts (Non-Latin first for better detection)
    // This catches Hindi, Arabic, Chinese, etc.
    for (const entry of entries) {
        if (entry.regex && entry.regex.toString() !== '/[a-zA-Z]/i' && entry.regex.test(text)) {
            return entry.code;
        }
    }

    // Priority 2: Latin Scripts with Unique Characters
    // If text is Latin, check for specific markers (e.g., Azerbaijani 'ə', Spanish 'ñ', Turkish 'ş')
    const latinEntries = entries.filter(e => e.regex && e.regex.toString().includes('a-zA-Z') && e.regex.toString() !== '/[a-zA-Z]/i');
    for (const entry of latinEntries) {
        if (entry.regex && entry.regex.test(text)) {
            return entry.code;
        }
    }

    // Priority 3: Default to English if strictly Latin (a-z) or unknown
    return 'en-US';
};
