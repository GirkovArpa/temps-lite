<h1 align="center">
<a href="https://github.com/girkovarpa/temps-lite">
<img src="app/assets/app.png" alt="temps-lite" width="200"/></a><br/><br/>
temps-lite
<br/>
<br/>
</h1>

<h4 align="center">A smart, good-looking little app which tries to speak your language the way you are used to.</h4>
<h5 align="center"></h5>

<h1 align="center">
<img src="preview.gif" alt="temps-lite" width="280"/></a><br/><br/>
</h1>

## Features 🚀

- [x] Small filesize, ~10mb
- [x] Light memory footprint
- [x] Actual weather information for any location
- [x] Four-Day weather forecast
- [x] Information directly in your taskbar available
- [x] _Rain_, _snow_ and _thunder_ animations
- [x] Minimal and colorful design
- [x] Draggable
- [x] Minimizable

## Translations 🌍

- [x] Dutch
- [x] English
- [x] French
- [x] German
- [x] Italian
- [x] Portuguese (BR)
- [x] Spanish

## Todo 📋

- [ ] Interactive hourly weather graph
- [ ] Timezone support
- [ ] Shortcut Key support
- [x] Autostart on login
- [ ] Move animations below forecasts layer
- [ ] Fix Apply/Quit button transitions

## Building ▶️

Compiling with Rust is fairly straightforward:
```sh
cargo build --release
```
When running, the folder [locale](locale) must be in the same folder as the executable, and the [Sciter](https://github.com/c-smile/sciter-sdk/tree/4b1a855f58e47f1677b28c0d8e9a17b6b448b3af) library from either `bin.win`, `bin.lnx`, or `bin.osx` must be in the folder as well, or in your `PATH`.

## Location

Normally it is very straightforward to enter your location into the settings window in temps-lite: just type TOWN, COUNTRY and click "Apply". But what, if you live in a place, the name of which exists several times in your own country? The solution: Go to [OpenweatherMap](https://openweathermap.org/find?q=#) for example and enter the name of your city. In the case of multiple search results for your country you will see the geocoorcinates for the individual search results with their names. Right-click on the desired one and select 'Copy link address' from the menu. Paste it into the settings window of temps-lite. Delete everything from the insertion except for the numbers separated either by a comma or a space and click 'Apply'. You should now see the weather in your location.

Even more granular: The weather above your home! Go for example to [OpenStreetMap](https://www.openstreetmap.org/#map=6/51.330/10.453) and enter your home address. The map of your location will appear. Look for your house/apartment and right-click on it. Select 'Show address' from the menu. You will see the geocoordinates of it on the left side of the window. Copy them and paste them into the settings window of temps-lite. Delete everything from the insertion except for the numbers separated by either a comma or a space. Click 'Apply', and that's it!

## Notes ⚠️

<details>
  <summary>Linux Font Installation</summary>
  
  Please manually install the following fonts from [this](app/assets/fonts/rubik/) folder:
  - **Rubik Regular 400**
  - **Rubik Light 300**
  - **Rubik Bold 700**
</details>

## History 📜

This application was forked from the original [temps](https://github.com/jackd248/temps) repository.  Unlike that project, this one is based on [Sciter](https://sciter.com/) instead of Electron.

The weather icons are made by the [original author](https://github.com/jackd248). If you would like to use them in your own project, you can find the icons on [Github](https://github.com/jackd248/weather-iconic) or at the [Noun Project](https://thenounproject.com/konradmichalik/collection/weather/).

## Contributors ✨

Thanks goes to these wonderful people:

<table>
  <tr>
    <td align="center"><a href="https://github.com/4silvertooth"><img src="https://avatars.githubusercontent.com/u/793967?v=3?s=100" width="100px;" alt=""/><br /><sub><b>4silvertooth</b></sub></a><br /><a href="https://github.com/GirkovArpa/temps-lite/commit/8a06af5921ea8005d49e9ac90e750ba912fa17cb" title="Code">💻</a><a href="https://github.com/GirkovArpa/temps-lite/commit/b977bb2327f797ec0b6ed1acd82f677ae96cfd9b" title="Linux">🐧</a> 
    <td align="center"><a href="https://github.com/papioara"><img src="https://avatars.githubusercontent.com/u/31970041?v=3?s=100" width="100px;" alt=""/><br /><sub><b>papioara</b></sub></a><br /><a href="https://github.com/GirkovArpa/temps-lite/issues/1#issuecomment-774692453" title="Translations">🌍</a><a href="https://github.com/GirkovArpa/temps-lite/pull/9" title="Bug Fixes">🐛</a><a href="https://github.com/GirkovArpa/temps-lite/issues/24#issuecomment-790610531" title="Linux">🐧</a> </td>
    <td align="center"><a href="https://github.com/logix2"><img src="https://avatars.githubusercontent.com/u/38005360?v=3?s=100" width="100px;" alt=""/><br /><sub><b>logix2</b></sub></a><br /><a href="https://github.com/GirkovArpa/temps-lite/issues/17#issuecomment-785044732" title="Linux">🐧</a><a href="https://www.linuxuprising.com/" title="Linux Uprising Blog">Linux Uprising Blog</a></td>
    <td align="center"><a href="https://github.com/goliv04053"><img src="https://avatars.githubusercontent.com/u/22873288?v=3?s=100" width="100px;" alt=""/><br /><sub><b>goliv04053</b></sub></a><br /><a href="https://github.com/GirkovArpa/temps-lite/commit/01f7a6d8500dac0df89f24bcb7430c9c265b6085" title="Translations">🌍</a></td>
  </tr>
</table>

## License [![License](https://img.shields.io/github/license/jackd248/temps.svg?style=flat-square)]()

Copyright © 2016 Konrad Michalik, 2021 Girkov Arpa. This software is licensed under the [MIT License](LICENSE).

