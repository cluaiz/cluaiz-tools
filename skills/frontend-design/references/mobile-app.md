# Mobile apps

For iOS and Android in React Native (Expo), Flutter, SwiftUI or Jetpack Compose. Follow each platform's conventions; web ideas do not carry over directly.

## Safe areas

Notches, cut-outs and the home indicator take space, so never hardcode top or bottom padding. React Native: `SafeAreaProvider` and `useSafeAreaInsets()` from `react-native-safe-area-context`. Flutter: `SafeArea` or `MediaQuery.paddingOf`. SwiftUI respects safe areas by default (use `.ignoresSafeArea()` only for backgrounds). Compose: `WindowInsets.safeDrawing` or the `Scaffold` insets. A bottom button sits above the home indicator.

## Touch and reach

- Targets are at least 44×44pt on iOS and 48×48dp on Android, even when the icon is smaller: `hitSlop`, Flutter `minimumSize` or padding, SwiftUI `.contentShape`, Compose `minimumInteractiveComponentSize()`.
- Primary actions sit within thumb reach, in the lower part of the screen. Destructive actions are kept away from frequent ones.

## Patterns people expect

Bottom sheets for choices and short tasks. Pull to refresh on feeds (`RefreshControl`, `RefreshIndicator`, `.refreshable`, `PullToRefreshBox`). Swipe actions on rows where the platform uses them. Back works the platform's way: the iOS edge swipe and the Android back gesture.

## iOS and Android differ

| | iOS (Human Interface Guidelines) | Android (Material 3) |
|---|---|---|
| Type | SF Pro, Dynamic Type styles | Roboto or the brand font, Material type scale |
| Top bar | Centred title, chevron back, large title that collapses | Left title, arrow back |
| Navigation | Tab bar | Navigation bar, or a rail on tablets |
| Confirm | Action sheet or alert | Dialog or bottom sheet |

Flutter: `Cupertino` widgets or `.adaptive` constructors where the app should feel native on iOS. React Native: `Platform.select` for the differences.

## Text, themes and accessibility

- Text follows the user's size setting (Dynamic Type, Android font scale), and layouts survive 200%.
- Light and dark come from the same tokens (`useColorScheme`, `ThemeData` brightness, asset catalogue colours, `MaterialTheme` colour schemes).
- Every control has a label for VoiceOver and TalkBack: `accessibilityLabel`, Flutter `Semantics` or `tooltip`, SwiftUI `.accessibilityLabel`, Compose `contentDescription`. Decorative images are hidden from screen readers.

## Performance

Virtualize long lists: `FlatList` or `FlashList` (not `map` inside a `ScrollView`), `ListView.builder`, `LazyVStack` or `List`, `LazyColumn`. Request images at the size shown and cache them. Keep forms above the keyboard (`KeyboardAvoidingView`, `Scaffold` resize, `imePadding()`). Keep heavy work off the UI thread (an isolate in Flutter, a background task elsewhere).

## Checks

The project's tests (Jest with React Native Testing Library, `flutter test`, XCTest and XCUITest, Compose UI tests) and linters (`flutter analyze`, ESLint and TypeScript, SwiftLint, ktlint). Check a small and a large phone, both themes, and large text. If the app cannot be run here, ask the user to run it, or say it was checked only in code.
