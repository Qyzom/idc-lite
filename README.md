# 🦀 RustCooling v1.0.0

> **Ультимативный, ультралегковесный кроссплатформенный CLI & демон для управления LCD-дисплеями систем жидкостного охлаждения ID-COOLING FX (FX240 / FX360).**

[![Rust](https://img.shields.io/badge/Rust-1.75+-DEA584?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-blue?style=for-the-badge)](README.md)

---

## 📖 Предыстория

**RustCooling** — это логическое завершение и полная перезагрузка проекта [idc-lite](https://github.com/Qyzom/idc-lite).  
Проект стал первым глубоким опытом автора в экосистеме **Rust**, объединив строгую безопасность работы с памятью, бескомпромиссную производительность системного уровня и отточенную архитектуру прямого взаимодействия с оборудованием без тяжелых рантаймов.

---

## ✨ Особенности

* ⚡ **Аномально низкое потребление RAM:** Всего **1.5 – 2.5 МБ** против ~70-100 МБ у C# WPF и 200+ МБ у вендорных утилит.
* 🛡️ **Zero Proprietary Drivers:** Никаких неподписанных `.sys` драйверов ядра (WinRing0) и проприетарщины. Только 100% безопасный и чистый Open-Source стек (**WMI / PDH** на Windows, **/sys/class/hwmon** на Linux).
* 📦 **Один монолитный бинарник:** Никаких сторонних DLL, зависимостей от громоздких фреймворков или виртуальных машин.
* ✨ **Плавные и исправленные анимации:** Полностью устранены зависания, подергивания и пропуски кадров оригинального проекта (реализованы алгоритмы **EaseInOutCubic**, **Roller**, а также мгновенные **Instant**-переходы).
* 🎨 **Красивый CLI:** Удобный, цветной терминальный интерфейс в стильной палитре **Catppuccin Mocha**.
* 🚀 **Надежная автозагрузка:** Нативная интеграция с **Task Scheduler** на Windows (без всплывающих консольных окон и повторных запросов UAC) и native **systemd user unit** на Linux.
* ⚖️ **Лицензия:** 100% **MIT License** — свободный и открытый исходный код.

---

## 📊 Таблица сравнения ресурсов

| Параметр | Оригинал ID-COOLING | IDC-Lite (C# / WPF) | 🦀 RustCooling v1.0.0 |
| :--- | :---: | :---: | :---: |
| **Потребление RAM** | 200+ МБ | ~70 – 100 МБ (20 МБ в трее) | **1.5 – 2.5 МБ** 🚀 |
| **Драйверы ядра** | Проприетарные / Закрытые | WinRing0 (`.sys`) | **Zero Drivers (WMI/PDH/hwmon)** |
| **Поддержка Linux** | ❌ Отсутствует | Частичная (C# Daemon) | **✅ Полная нативная (CLI + Daemon)** |
| **Зависимости** | Electron / Node.js / C++ runtime | .NET 8.0 Runtime | **Ноль (Self-contained static binary)** |
| **Плавность анимаций** | ❌ Нет анимаций | Базовая (иногда фризы) | **Идеальная (60 FPS, Cubic / Roller)** |
| **Размер дистрибутива** | ~150 МБ | ~25–30 МБ | **~4–8 МБ** |
| **Лицензия** | Proprietary | MIT | **MIT** |

---

## 🚀 Быстрый старт

### Запуск мониторинга

Запуск отображения температуры процессора с плавной анимацией:

```bash
# Базовый запуск (по умолчанию температура CPU)
rustcooling

# Или явный вызов подкоманды run
rustcooling run --mode cpu-temp --anim smooth --interval 1000
```

### Основные флаги и параметры

```text
Usage: rustcooling [OPTIONS] [COMMAND]

Commands:
  run         Запуск цикла мониторинга и вывода на LCD (действие по умолчанию)
  autostart   Управление фоновым автозапуском программы
  help        Вывод справочной информации

Options:
  -m, --mode <MODE>              Режим отображения [cpu-temp, cpu-load, cpu-freq] [default: cpu-temp]
  -a, --anim <ANIM>              Режим анимации: instant, smooth, roller [default: smooth]
  -i, --interval <INTERVAL>      Интервал опроса телеметрии в миллисекундах [default: 1000]
  -d, --anim-duration <MS>       Длительность перехода анимации в мс [default: 400]
      --daemon                   Фоновый/тихий режим без интерактивного TUI
  -h, --help                     Справка
  -V, --version                  Версия
```

### Управление автозагрузкой

```bash
# Включить автозапуск при входе в систему
rustcooling autostart --enable

# Отключить автозапуск
rustcooling autostart --disable

# Проверить статус автозапуска
rustcooling autostart --status
```

---

## 🛠️ Сборка из исходников и упаковка

### Требования

* **Rust & Cargo** (версии 1.75+)
* На Linux: `libudev-dev` и `pkg-config`

```bash
# Установка зависимостей на Debian / Ubuntu
sudo apt-get update && sudo apt-get install -y build-essential libudev-dev pkg-config
```

### Сборка бинарного файла

```bash
git clone https://github.com/Qyzom/RustCooling.git
cd RustCooling

# Компиляция оптимизированного релизного бинарника
cargo build --release
```

Готовый бинарник будет находиться по пути: `target/release/rustcooling` (или `target/release/rustcooling.exe` на Windows).

### Упаковка в `.deb` пакет (Linux)

Для удобной установки в системах на базе Debian/Ubuntu используйте подготовленные шаблоны упаковки:

```bash
# 1. Создаем структуру каталогов
mkdir -p deb_pkg/usr/bin
mkdir -p deb_pkg/usr/lib/udev/rules.d
mkdir -p deb_pkg/usr/lib/systemd/system
mkdir -p deb_pkg/DEBIAN

# 2. Копируем файлы
cp target/release/rustcooling deb_pkg/usr/bin/
cp packaging/99-idcooling.rules deb_pkg/usr/lib/udev/rules.d/
cp packaging/rustcooling.service deb_pkg/usr/lib/systemd/system/
cp packaging/deb/control deb_pkg/DEBIAN/
cp packaging/deb/postinst deb_pkg/DEBIAN/
cp packaging/deb/prerm deb_pkg/DEBIAN/

# 3. Выставляем права и собираем пакет
chmod 755 deb_pkg/usr/bin/rustcooling deb_pkg/DEBIAN/postinst deb_pkg/DEBIAN/prerm
chmod 644 deb_pkg/DEBIAN/control deb_pkg/usr/lib/udev/rules.d/99-idcooling.rules deb_pkg/usr/lib/systemd/system/rustcooling.service

dpkg-deb --build deb_pkg rustcooling_1.0.0_amd64.deb

# 4. Установка собранного пакета
sudo dpkg -i rustcooling_1.0.0_amd64.deb
```

---

## 📄 Лицензия

Проект распространяется под лицензией [MIT](LICENSE).
