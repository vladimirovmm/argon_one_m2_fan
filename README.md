# Argon One M2 Fan
Запуск и управление вентилятором для **Argon One M2** на **Ubuntu Sever 22.04**.

Работает без sudo.

## Определение адреса вентилятора:

Поиск шины к которому подключён вентилятор

```bash
i2cdetect -l
```

Вывод:
```output
i2c-1  ...
```
**i2c-N** - где _N_ это номер шины

### Просмотр информации о шине
```bash 
i2cdetect -F 1
```
### Ищем адрес вентилятора

```bash
i2cdetect -y 1
```
Вывод:
```code 
0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:                         -- -- -- -- -- -- -- --
10: -- -- -- -- -- -- -- -- -- -- 1a -- -- -- -- --
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
70: -- -- -- -- -- -- -- --
```
**1a** - это адрес вентилятора

## Пробуем включить вентилятор
Вентилятор принемет значение от 0 до 100
```bash
i2cset 1 0x1a 100
```

## Настройка перед сборки бинарника

src/main.rs

```code
const I2C_PATH: &str = "/dev/i2c-N";
```
_N_ - номер шины

```code
const ADDRESS: u16 = 0x1a;
```
_0x1a_ - адрес вентилятора

```code
const MIN_TEMPERATURE: u8 = 50;
```
_50_ - Минимальная температура. Если температура меньше или равна то скорость вентилятора 0

```code
const MAX_TEMPERATURE: u8 = 65;
```
_65_ - Максимальная температура.

```code
const TIMEOUT_SEC: u8 = 1;
```
_1_ - Частота обновления показаний в секундах

### Сборка

Необходимо запустить в корне проекта
```bash
cargo build --release
```

По завершению сборки бинарник для запуска будет лежать в _./target/release/argon_m2_fan_i2c_

Запуск:
```bash
./target/release/argon_m2_fan_i2c
```


