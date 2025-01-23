# clockem ("clock 'em")
Clock and other desktop widgets for Wayland.

Current functions are:
- clock widget
- weather
- wallpaper

Reach out to me through this repo or on discord at `justbechill` if you have widget suggestions/requests.

## Installing
There is an aur package for clockem:
```
yay -S clockem-git
```

## Example
![image](https://github.com/user-attachments/assets/1bbc2ff1-c1d5-4895-8a21-f6e4608cba2f)
Send me your screenshots to be included in the examples!

## Configuration
Configuration files consist of `config.toml` and `style.css` in `~/.config/clockem`. If no config files are found, [default configs](https://github.com/JustBeChill/clockem/tree/main/default-configs) will be generated in the correct directories.

**Clock Widget**
|       Key       |     Value     | Default        |  Description  |
| --------------- | ------------- | ------------- | ------------- |
|`enabled`        |`boolean`      |`false`        |Enable/disable widget       |
|`top_format`     |`String`       |`%H:%M:%S`     |Date/time format string for top element, using [chrono's string formatting specifiers](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers)|
|`bottom_format`  |`String`       |`%b %d %Y`    |Date/time format string for bottom element, using [chrono's string formatting specifiers](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers)|               |
|`position_x`     |`int`          |`0`            |X position of widget from the top of the display.|
|`position_y`     |`int`          |`0`            |Y position of widget form the top of the display.|
|`y_align`        |`String`       |`"top"`        |Options are "center", "bottom", and "top"; overrides `position_y` if not `"top"`.|
|`update_interval`|`uint`         |`1`            |Interval in seconds between clock updates|

**Wallpaper** - For now, the wallpaper image will always display at full scale, so it must be the resolution of your display to appear properly.
|      Key      |     Value     | Default         |  Description  |
| ------------- | ------------- | --------------- | ------------- |
|`enabled`      |`boolean`      |`false`          |Enable/disable widget       |
|`directory`    |`String`       |`empty`          |Path to wallpaper|
|`vert_adjustment`|`int`       |`0`               |May need to be used if waybar or a similar program is causing a gap to appear between wallpaper and the top of the display.|

## Weather Format Strings
|   Key   |   Value  |
| ------- | -------- |
|`%L`     |City or location name                 |
|`%l`     |Country name                          |
|`%R`     |Region name (state, territory, etc.)  |
|`%S`     |Weather condition (e.g. Clear, Cloudy)|
|`%F`     |Actual temp in Fahrenheit             |
|`%f`     |"Feels like" temp in Fahrenheit       |
|`%C`     |Actual temp in Celcius                |
|`%c`     |"Feels like" temp in Celcius          |
|`%K`     |Actual temp in Kelvin                 |
|`%k`     |"Feels like" temp in Kelvin           |
|`%W`     |Wind speed in kph                     |
|`%w`     |Wind speed in mph                     |
|`%D`     |Wind direction, cardinal              |
|`%d`     |Wind direction, degrees               |
|`%P`     |Precipitation in mm                   |
|`%p`     |Precepitation in inches               |
|`%H`     |Percent humidity                      |
|`%D`     |Day or night, sets to `daynight_strings` config option |
|`%U`     |UV index                              |
