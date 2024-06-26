# Change Log

## 2024.05.27 - v0.4.0

- Added support for environment options [RFC1572](https://www.rfc-editor.org/rfc/rfc1572.html). [@not-jan](https://github.com/not-jan)
- Added support for binary transmission [RFC856](https://www.rfc-editor.org/rfc/rfc856.html). [@not-jan](https://github.com/not-jan)
- Added support for Unicode characters in character-at-a-time mode. This is gated behind the `unicode` feature. [@not-jan](https://github.com/not-jan)

## 2024.04.07 - v0.3.0

- Added support for character-at-a-time mode and line mode.
  [@not-jan](https://github.com/not-jan)
  - Character-at-a-time mode can be enabled by setting the `message_mode` field
    in the `TelnetCodec` struct to `false`. This will result in each character
    generating an individual event.
- Exposed the full `TelnetCodec` struct to the public API, where previously it
  was only partially exposed.

## 2024.03.20 - v0.2.0

- Fixes several off-by-one errors in the decoding logic.
  [@xxx](https://github.com/xxx)
- Added raw message support in the TelnetEvent enum.
  [@xxx](https://github.com/xxx)
- Added tests for decoding logic. [@xxx](https://github.com/xxx)
- General documentation improvements.

## 2023.02.18 - v0.1.0

Initial release.
