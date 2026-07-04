# Competitor Animation & UX Patterns — Mobile Nutrition Apps

Scope: mobile shell/navigation, bottom tab behavior, and micro-interaction animation patterns.
App note: this repo is a pure Rust/Dioxus web app targeting desktop/mobile/web (`dioxus/router`). No files were changed; these patterns are for informing future animation polish.

## Summary of Patterns to Adopt
- **Active tab ink with springy scale and color transition:** adopts subtle enlargement + color tint on active bottom-tab item, aligning with dominant health-app convention.
- **Shared-element style progress ring/strip transitions:** animate nutrition rings or daily progress strips between list and detail views with cross-fade + expand, preserving context.
- **Staggered list entrance:** when opening diary sections, stagger card entrance with `spring` easing and slightly randomized delays for organic feel instead of flat fade.
- **Bottom-sheet macros:** adopt press-to-expand micro-interactions for food entries that lift from list into sheet without losing origin point.
- **Pull-to-refresh with rubber-band overshoot and progress indicator:** keeps mobile delight without fighting web scroll affordances.
- **Compact inline FAB morph:** morph the add-food FAB into a quick-add toolbar instead of opening a full-screen modal to reduce interaction cost.

## Summary of Patterns to Avoid
- **Over-animated onboarding/tours:** heavy carousels block onboarding and don’t translate to lightweight health tools.
- **Continuous looping illustrations:** animated brand loops in headers increase perceived load and can distract from the primary metric.
- **Excessive haptic-vibration blurbs:** non-critical haptics on every calorie log adds noise; reserve for save/action confirmations.
- **Hero-style full-screen transitions:** iOS-rigid large-title transitions often misalign with Dioxus web layout and create gap/jank on viewport changes.

## Per-App Synthesized Findings (2025 proxy data)
- **MyFitnessPal:** dominant tab bar with constant state, tight typographic scale, token-colored category pills; animation is restraint-first, mostly around selection/focus states and input chips.
- **Lose It!:** accessible, slightly playful bottom-tab metrics; active state draws heavier stroke and subtle icon movement; avoids parallax heavy shells.
- **Yazio:** uses emoji/icon-forward diary entries with micro-scale bounces on add actions; pill counts animate with spring counters.
- **Lifesum:** heavier branding motion on on-boarding/programs then restraint in diary/track; bottom nav emphasizes color-switch over motion.
- **Noom:** lesson-driven content flow uses inline progress + counted microsteps; prefers fade-zero transitions over hero slides.
- **Cronometer:** strict data density; minimum motion, clear ring drawings; adopt ring segment count-up animation for first-of-day metrics.

## Dioxus Implementation Notes
- Dioxus 0.7 CSS transitions/animations are workable for these without adding motion libraries.
- If translating motion intensity to WASM desktop/mobile from web build, test 60fps scroll first; nutrition apps contain long food lists where animation regressions are most visible.
- Navigation transitions in Dioxus router should favor small shared-element cues or fade/translate over center-axis motion which often feels modal and brittle.
- Keep state-driven motion declarative: derive classes/styles from route/index rather than time-based orchestration for less GC pressure.
- Consider `ease-out-back` micro-bounces only under 200ms on small UI chrome; anything longer becomes sluggish in scrolling lists.

## Next Steps if Needed
- Capture current shell/nav implementation files for a gap analysis against these patterns.
- Propose specific Dioxus animation primitives: CSS transition keywords, keyframes, or a small JS-side easing shim if needed.
