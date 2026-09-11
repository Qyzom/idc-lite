# RustCooling

Кроссплатформенная CLI-утилита и фоновый демон для управления дисплеями СЖО ID-COOLING серии FX (FX240, FX280, FX360).

Проект является полной заменой устаревшей утилиты `idc-lite`. Кодовая база переписана на Rust для достижения детерминированного потребления системных ресурсов, устранения зависимостей от тяжелых рантаймов (.NET, WPF, Electron) и отказа от сторонних драйверов ядра.

## Технические характеристики

- **Потребление ОЗУ:** 1.5 – 2.5 МБ (в отличие от 45–80 МБ в .NET-версии).
- **Размер исполняемого файла:** ~750 КБ (статическая линковка, LTO, strip).
- **Драйверы:** не требуются. Работа с железом осуществляется через стандартные интерфейсы операционной системы.
- **Поддерживаемые ОС:** Linux (любые дистрибутивы с glibc/musl), Windows 10/11 (x64).
- **Протокол:** HID (VID `0x3402`, PID `0x0100`), 64-байтные репорты.
- **Лицензия:** MIT.

## Архитектура источников метрик

| ОС | Загрузка CPU | Частота CPU | Температура CPU |
| :--- | :--- | :--- | :--- |
| **Linux** | `/proc/stat` (дельта jiffies) | `/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq` или `/proc/cpuinfo` | `/sys/class/hwmon` (`k10temp`, `coretemp`, `zenpower`, `cpu_thermal`) |
| **Windows** | Win32 API `GetSystemTimes` | WMI `Win32_Processor` (`CurrentClockSpeed`) | WMI `MSAcpi_ThermalZoneTemperature` / `Win32_PerfFormattedData_Counters_ThermalZoneInformation` |

## Использование

### Запуск мониторинга в терминале
```bash
rustcooling run
```

Параметры запуска:
- `-i, --interval <MS>`: интервал опроса датчиков и обновления экрана в миллисекундах (по умолчанию: `1000`).
- `-a, --animation <MODE>`: режим анимации перехода значений: `smooth` (интерполяция EaseInOutCubic), `roller` (пошаговая прокрутка), `none` (мгновенно). По умолчанию: `smooth`.
- `-d, --duration <MS>`: длительность перехода анимации в миллисекундах (по умолчанию: `300`).

### Фоновый режим (демон)
Запуск процесса без вывода интерфейса в stdout (подходит для работы в качестве системной службы):
```bash
rustcooling run --daemon
```

### Диагностика состояния
Проверка связи с контроллером дисплея и доступности системных датчиков:
```bash
rustcooling status
```

### Управление автозагрузкой

Регистрация в планировщике системы:
```bash
rustcooling autostart enable
```
- **Windows:** создание задачи в Планировщике задач (Task Scheduler) при входе пользователя без запросов повышенных привилегий (UAC).
- **Linux:** создание и активация пользовательского сервиса `systemd` (`~/.config/systemd/user/rustcooling.service`).

Проверка статуса:
```bash
rustcooling autostart status
```

Удаление из автозагрузки:
```bash
rustcooling autostart disable
```

## Настройка прав доступа на Linux (udev)

Для прямого доступа к устройству без `sudo` добавьте правило udev:

```bash
sudo cp packaging/99-idcooling.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

## Сборка из исходников

Требуется Rust 1.75+:

```bash
cargo build --release
```
Готовый бинарный файл находится в `target/release/rustcooling` (или `rustcooling.exe` на Windows).
