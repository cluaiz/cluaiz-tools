import { detectLanguageFromText, UNIVERSAL_VOICE_REGISTRY } from './UniversalVoiceRegistry';

export { detectLanguageFromText };

export interface LanguageInfo {
    code: string;       // BCP-47 code
    name: string;       // Native name
    displayName: string; // English name
    flag: string;       // Emoji flag
    regex?: RegExp;     // Character block detection
    sampleText: string; // Testing sample
}

// Helper to get a high-quality sample text for any language
const getSampleText = (code: string, name: string): string => {
    const samples: Record<string, string> = {
        'af-ZA': "Hallo! Ek is Cluaiz. Ek kan jou taal verstaan en vlot praat.",
        'am-ET': "ሰላም! እኔ ክሉአይዝ ነኝ። ቋንቋህን መረዳት እና አቀልጥፌ መናገር እችላለሁ።",
        'ar-EG': "مرحباً! أنا كلويز. أستطيع فهم لغتك والتحدث بها بطلاقة.",
        'ar-SA': "مرحباً! أنا كلويز. أستطيع فهم لغتك والتحدث بها بطلاقة.",
        'as-IN': "নমস্কাৰ! মই ক্লুয়াইজ। মই আপোনাৰ ভাষা বুজিব পাৰোঁ আৰু সাবলীলভাৱে ক'ব পাৰোঁ।",
        'az-AZ': "Salam! Mən Cluaiz-əm. Mən sizin dilinizi başa düşə və səlis danışa bilirəm.",
        'be-BY': "Прывітанне! Я Cluaiz. Я магу разумець вашу мову і свабодна на ёй размаўляць.",
        'bg-BG': "Здравейте! Аз съм Cluaiz. Мога да разбирам вашия език и да го говоря свободно.",
        'bn-BD': "হ্যালো! আমি ক্লুয়াইজ। আমি আপনার ভাষা বুঝতে এবং সাবলীলভাবে বলতে পারি।",
        'bn-IN': "হ্যালো! আমি ক্লુয়াইজ। আমি আপনার ভাষা বুঝতে এবং সাবলীলভাবে বলতে পারি।",
        'bs-BA': "Zdravo! Ja sam Cluaiz. Mogu razumjeti vaš jezik i tečno ga govoriti.",
        'ca-ES': "Hola! Sóc Cluaiz. Puc entendre el teu idioma i parlar-lo amb fluïdesa.",
        'cs-CZ': "Ahoj! Jsem Cluaiz. Rozumím vašemu jazyku a mluvím jím plynně.",
        'cy-GB': "Helo! Cluaiz ydw i. Gallaf ddeall eich iaith a'i siarad yn rhugl.",
        'da-DK': "Hej! Jeg er Cluaiz. Jeg kan forstå dit sprog og tale det flydende.",
        'de-DE': "Hallo! Ich bin Cluaiz. Ich kann Ihre Sprache verstehen en fließend sprechen.",
        'el-GR': "Γεια σας! Είμαι ο/η Cluaiz. Μπορώ να καταλάβω τη γλώσσα σας και να τη μιλήσω άπταιστα.",
        'en-US': "Hello! I am Cluaiz. I can understand and speak your language fluently.",
        'es-ES': "¡Hola! Soy Cluaiz. Puedo entender y hablar tu idioma con fluidez.",
        'et-EE': "Tere! Mina olen Cluaiz. Ma mõistan teie keelt ja räägin seda vabalt.",
        'fa-IR': "سلام! من کلوایز هستم. من می‌توانم زبان شما را بفهمم و به روانی صحبت کنم.",
        'fi-FI': "Hei! Olen Cluaiz. Ymmärrän kieltäsi ja puhun sitä sujuvasti.",
        'fil-PH': "Halo! Ako si Cluaiz. Kaya kong intindihin ang iyong wika at magsalita nito nang matatas.",
        'fr-FR': "Bonjour ! Je suis Cluaiz. Je peux comprendre et parler votre langue couramment.",
        'gu-IN': "નમસ્તે! હું ક્લુઇઝ છું. હું તમારી ભાષા સમજી શકું છું અને અસ્ખલિત રીતે બોલી શકું છું.",
        'he-IL': "שלום! אני Cluaiz. אני יכול להבין את השפה שלך ולדבר אותה שוטף.",
        'hi-IN': "नमस्ते! मैं क्लुआइज़ (Cluaiz) हूँ। मैं आपकी भाषा समझ सकती हूँ और बोल भी सकती हूँ।",
        'hr-HR': "Zdravo! Ja sam Cluaiz. Mogu razumjeti vaš jezik i tečno ga govoriti.",
        'hu-HU': "Szia! Cluaiz vagyok. Értem a nyelvedet, és folyékonyan beszélem.",
        'hy-AM': "Բարև ձեզ: Ես Cluaiz-ն եմ: Ես կարող եմ հասկանալ ձեր լեզուն և սահուն խոսել դրանով:",
        'id-ID': "Halo! Saya Cluaiz. Saya bisa mengerti bahasa Anda dan berbicara dengan lancar.",
        'is-IS': "Hæ! Ég er Cluaiz. Ég get skilið tungumálið þitt og talað það reitrennandi.",
        'it-IT': "Ciao! Sono Cluaiz. Posso capire e parlare la tua lingua fluentemente.",
        'ja-JP': "こんにちは！私はCluaizです。あなたの言語を理解し、流暢に話すことができます。",
        'jv-ID': "Halo! Aku Cluaiz. Aku bisa ngerti basamu lan ngomong kanthi lancar.",
        'ka-GE': "გამარჯობა! მე ვარ კლუაიზი. მე შემიძლია გავიგო თქვენი ენა და ვილაპარაკო თავისუფლად.",
        'kk-KZ': "Сәлем! Мен Cluaiz-бын. Мен сіздің тіліңізді түсінемін және еркін сөйлей аламын.",
        'km-KH': "សួស្តី! ខ្ញុំគឺ Cluaiz។ ខ្ញុំអាចយល់ភាសារបស់អ្នក និងនិយាយបានយ៉ាងស្ទាត់ជំនាញ។",
        'kn-IN': "ನಮಸ್ತೆ! ನಾನು ಕ್ಲುವೈಜ್. ನಿಮ್ಮ ಭಾಷೆಯನ್ನು ನಾನು ಅರ್ಥಮಾಡಿಕೊಳ್ಳಬಲ್ಲೆ ಮತ್ತು ನಿರರ್ಗಳವಾಗಿ ಮಾತನಾಡಬಲ್ಲೆ.",
        'ko-KR': "안녕하세요! 저는 Cluaiz입니다. 저는 당신의 언어를 이해하고 유창하게 말할 수 있습니다.",
        'lo-LA': "ສະບາຍດີ! ຂ້ອຍແມ່ນ Cluaiz. ຂ້ອຍສາມາດເຂົ້າໃຈພາສາຂອງເຈົ້າ ແລະ ເວົ້າໄດ້ຢ່າງລ່ຽນໄຫຼ.",
        'lt-LT': "Labas! Aš esu Cluaiz. Galiu suprasti jūsų kalbą ir sklandžiai ja kalbėti.",
        'lv-LV': "Sveiki! Es esmu Cluaiz. Es saprotu jūsu valodu un varu tajā brīvi runāt.",
        'mk-MK': "Здраво! Јас сум Cluaiz. Можам да го разберам вашиот јазик и да го зборувам течно.",
        'ml-IN': "ഹലോ! ഞാൻ ക്ലുഅയിസ് ആണ്. എനിക്ക് നിങ്ങളുടെ ഭാഷ മനസ്സിലാക്കാനും ഒഴുക്കോടെ സംസാരിക്കാനും കഴിയും.",
        'mn-MN': "Сайн байна уу? Би Cluaiz байна. Би таны хэлийг ойлгож, чөлөөтэй ярьж чадна.",
        'mr-IN': "नमस्कार! मी क्लुआईझ आहे. मी तुमची भाषा समजू शकते आणि अस्खलितपणे बोलू शकते.",
        'ms-MY': "Helo! Saya Cluaiz. Saya boleh faham bahasa anda dan bercakap dengan lancar.",
        'my-MM': "မင်္ဂလာပါ! ကျွန်တော် Cluaiz ပါ။ ကျွန်တော် ခင်ဗျားတို့ဘာသာစကားကို နားလည်နိုင်ပြီး ကျွမ်းကျင်စွာ ပြောဆိုနိုင်ပါတယ်။",
        'ne-NP': "नमस्ते! म क्लुआइज हुँ। म तपाईंको भाषा बुझ्न सक्छु र धाराप्रवाह बोल्न सक्छु।",
        'nl-NL': "Hallo! Ik ben Cluaiz. Ik kan uw taal begrijpen en vloeiend spreken.",
        'pa-IN': "ਸਤਿ ਸ੍ਰੀ ਅਕਾਲ! ਮੈਂ ਕਲੂਆਇਜ਼ ਹਾਂ। ਮੈਂ ਤੁਹਾਡੀ ਭਾਸ਼ਾ ਨੂੰ ਸਮਝ ਸਕਦਾ ਹਾਂ ਅਤੇ ਰਵਾਨਗੀ ਨਾਲ ਬੋਲ ਸਕਦਾ ਹਾਂ।",
        'pl-PL': "Cześć! Jestem Cluaiz. Rozumiem Twój język i mówię nim biegle.",
        'ps-AF': "سلام! زه کلوایز یم. زه ستاسو په ژبه پوهیدلی شم او په روانه توګه خبرې کولی شم.",
        'pt-BR': "Olá! Eu sou o Cluaiz. Posso entender e falar seu idioma fluentemente.",
        'ro-RO': "Bună! Sunt Cluaiz. Pot înțelege limba ta și o pot vorbi fluent.",
        'ru-RU': "Привет! Я Cluaiz. Я могу понимать и свободно говорить на вашем языке.",
        'si-LK': "ආයුබෝවන්! මම ක්ලුවයිස්. මට ඔබේ භාෂාව තේරුම් ගත හැකි අතර චතුර ලෙස කතා කළ හැකිය.",
        'sk-SK': "Ahoj! Som Cluaiz. Rozumiem vášmu jazyku a hovorím ním plynule.",
        'sl-SI': "Živjo! Sem Cluaiz. Razumem vaš jezik in ga tekoče govorim.",
        'sq-AL': "Përshëndetje! Unë jam Cluaiz. Mund ta kuptoj gjuhën tuaj dhe ta flas rrjedhshëm.",
        'sr-RS': "Здраво! Ја сам Cluaiz. Могу да разумем ваш језик и да га течно говорим.",
        'su-ID': "Halo! Simkuring Cluaiz. Simkuring tiasa ngartos basa anjeun sarta nyarita kalawan lancar.",
        'sv-SE': "Hej! Jag är Cluaiz. Jag kan förstå ditt språk och tala det flytande.",
        'sw-KE': "Hujambo! Mimi ni Cluaiz. Naweza kuelewa lugha yako na kuizungumza kwa ufasaha.",
        'ta-IN': "வணக்கம்! நான் கிளூயைஸ். உங்கள் மொழியை என்னால் புரிந்து கொள்ளவும் சரளமாக பேசவும் முடியும்.",
        'te-IN': "హలో! నేను క్లూయిజ్. నేను మీ భాషను అర్థం చేసుకోగలను మరియు అనర్గళంగా మాట్లాడగలను.",
        'th-TH': "สวัสดี! ฉันชื่อ Cluaiz ฉันสามารถเข้าใจและพูดภาษาของคุณได้อย่างคล่องแคล่ว",
        'tr-TR': "Merhaba! Ben Cluaiz. Dilinizi anlayabilir ve akıcı bir şekilde konuşabilirim.",
        'uk-UA': "Привіт! Я Cluaiz. Я можу розуміти вашу мову і вільно нею розмовляти.",
        'ur-PK': "ہیلو! میں کلوائز ہوں۔ میں آپ کی زبان سمجھ سکتا ہوں اور روانی سے بول سکتا ہوں۔",
        'ur-IN': "ہیلو! میں کلوائز ہوں۔ میں آپ کی زبان سمجھ سکتا ہوں اور روانی سے بول سکتا ہوں-",
        'uz-UZ': "Salom! Men Cluaiz-man. Men sizning tilingizni tushunaman va ravon gapira olaman.",
        'vi-VN': "Xin chào! Tôi là Cluaiz. Tôi có thể hiểu ngôn ngữ của bạn và nói trôi chảy.",
        'zh-CN': "你好！我是 Cluaiz。我能理解并流利地呈现你的语言。",
        'zu-ZA': "Sawubona! NginguCluaiz. Ngiyakwazi ukuqonda ulimi lwakho futhi ngilukhulume kahle.",
        'yo-NG': "Pẹlẹ o! Emi ni Cluaiz. Mo le loye ede rẹ ati sọ ọ ni gbangba.",
        'ha-NE': "Sannu! Ni ne Cluaiz. Ina iya fahimtar yarenku kuma in yi magana da shi sarai.",
        'ig-NG': "Ndêwó! Abụ m Cluaiz. Enwere m ike ịghọta asụsụ gị wee kwuo ya nke ọma.",
        'or-IN': "ନମସ୍କାର! ମୁଁ କ୍ଲୁଆଇଜ୍ | ମୁଁ ଆପଣଙ୍କ ଭାଷା ବୁଝିପାରେ ଏବଂ ଅନର୍ଗଳ ଭାବରେ କହିପାରେ |",
        'ks-IN': "سلام! مۍ چُھ کلوایز۔ مۍ چُھ تُہنٛز زَبانہٕ سَمَجھ یِوان تہٕ مۍ چُھ روانی سان بولنہٕ یِوان۔",
        'sd-IN': "سلام! مان ڪلويز آهيان. مان توهان جي ٻولي سمجهي سگهان ٿو ۽ رواني سان ڳالهائي سگهان ٿو.",
        'mai-IN': "नमस्ते! हम क्लुआइज छी। हम अहाँक भाषा बुझि सकै छी आ धाराप्रवाह बाजि सकै छी।",
        'kok-IN': "नमस्कार! हांव क्लुआइज. माका तुजी भास समजता आनी हांव ती बरेतरेन उलयता.",
        'doi-IN': "नमस्ते! मैं क्लुआइज हां। मैं तुंदी भाशा समझी सकदा हां ते प्रवाह कन्नै बोली सकदा हां।",
        'brx-IN': "खुलुमबाय! आं क्लुआइज। आं नोंथांनि रावखौ बुजिनो हागौ आरो मोजाङै बुंनो हागौ।",
        'bho-IN': "नमस्ते! हम क्लुआइज बानी। हम रउआ भाषा समझ सकेनी अउर नीक से बोल सकेनी।",
    };

    if (samples[code]) return samples[code];

    const primaryCode = code.split('-')[0];
    const regionalFallback = Object.keys(samples).find(k => k.startsWith(primaryCode));
    if (regionalFallback) return samples[regionalFallback];

    return `Hello! This is Cluaiz speaking ${name}. I am a cross-platform AI assistant designed to support every culture on Earth.`;
};

/**
 * Automatically generate SUPPORTED_LANGUAGES from the full UNIVERSAL_VOICE_REGISTRY
 */
export const SUPPORTED_LANGUAGES: LanguageInfo[] = Object.keys(UNIVERSAL_VOICE_REGISTRY).map(code => {
    const meta = UNIVERSAL_VOICE_REGISTRY[code];
    return {
        ...meta,
        sampleText: getSampleText(code, meta.displayName)
    };
});

export const getLanguageByCode = (code: string) => {
    return SUPPORTED_LANGUAGES.find(l => l.code === code) || UNIVERSAL_VOICE_REGISTRY[code];
};
