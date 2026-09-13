<div align="center">

# ⚠️ IDC-Lite (ARCHIVED / DEPRECATED)

### This project has been discontinued and completely rewritten in Rust as [RustCooling](https://github.com/Qyzom/RustCooling).
### Этот проект устарел и полностью переписан с нуля на Rust: [RustCooling](https://github.com/Qyzom/RustCooling).

[![Superseded by RustCooling](https://img.shields.io/badge/Superseded_by-RustCooling-brightgreen?style=for-the-badge&logo=rust)](https://github.com/Qyzom/RustCooling)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

</div>

---

## English

> [!IMPORTANT]
> **IDC-Lite is officially archived.**  
> Please use **[RustCooling](https://github.com/Qyzom/RustCooling)** — an ultra-lightweight, 100% native Rust rewrite with zero dependencies, native Linux support, and an ultra-lean memory footprint (< 15 MB RAM open, < 5 MB in tray).

### Why was IDC-Lite rewritten?
**IDC-Lite** was the author's very first project created to escape the heavy vendor Electron software for ID-COOLING FX Series liquid coolers. While it achieved its goal, it was built using C# / .NET 8, required WinRing0 driver components on Windows, and had experimental, cumbersome Linux support with separate daemon scripts.

**[RustCooling](https://github.com/Qyzom/RustCooling)** solves all limitations:
- **100% Pure Rust:** Single standalone monolithic binary for both GUI and CLI daemon mode.
- **Native Linux Support:** Zero compromises, no proprietary kernel drivers or wrappers — uses direct kernel `hwmon`, `sysfs`, and `udev` rules.
- **Minimal RAM Footprint:** Drops from ~60–80 MB down to **< 15 MB** (and **< 5 MB** in system tray).
- **Zero Proprietary Drivers:** No WinRing0 or unsafe driver loading.
- **Instant Launch:** Starts in under 50 ms with a hardware-accelerated Slint UI.

👉 **Download the new Rust release:** [https://github.com/Qyzom/RustCooling/releases](https://github.com/Qyzom/RustCooling/releases)

---

## Русский

> [!IMPORTANT]
> **Разработка IDC-Lite завершена.**  
> Проект полностью переписан с нуля на языке Rust в новом репозитории: **[RustCooling](https://github.com/Qyzom/RustCooling)**.

### История и причины переписывания
**IDC-Lite** был моим самым первым проектом, созданным как альтернатива громоздкому и медленному официальному софту ID-COOLING на Electron. Однако первая версия базировалась на C# / .NET 8, использовала драйвер WinRing0 для Windows, а поддержка Linux была экспериментальной и требовала сторонних костылей.

Новая версия — **[RustCooling](https://github.com/Qyzom/RustCooling)** — полностью устраняет все эти недостатки:
- **100% нативный Rust:** Один легковесный бинарный файл, включающий графический интерфейс, системный трей и фоновый демон (`--daemon`).
- **Честная поддержка Linux без костылей:** Нативное чтение `hwmon`, `sysfs` и правила `udev`. Никакого проприетарного мусора или сторонних библиотек.
- **Потребление ОЗУ < 15 МБ:** В трее приложение потребляет **менее 5 МБ** (вместо 60–80 МБ в IDC-Lite и 200+ МБ в софте вендора).
- **Безопасность:** Отказ от небезопасного Ring0-драйвера в пользу стабильных системных интерфейсов.
- **Мгновенный запуск:** Скорость холодного старта < 50 мс на движке Slint UI с аппаратным ускорением FemtoVG OpenGL.

👉 **Перейти к актуальной версии:** [https://github.com/Qyzom/RustCooling](https://github.com/Qyzom/RustCooling)

---

## Сравнение поколений / Evolution Comparison

| Параметр | Оригинал ID-COOLING | IDC-Lite (C# / .NET 8) | [RustCooling](https://github.com/Qyzom/RustCooling) (Rust) |
| :--- | :---: | :---: | :---: |
| **Стек технологий** | Electron / Node.js | C# / .NET 8 (WPF) | **100% Pure Rust + Slint UI** |
| **Потребление ОЗУ** | ~180 – 300 МБ | ~50 – 80 МБ | **< 15 МБ (< 5 МБ в трее)** |
| **Размер дистрибутива** | ~150 МБ | ~25 – 30 МБ | **~12 МБ (всё включено)** |
| **Драйверы / Ядро** | Закрытый Ring0 драйвер | WinRing0.sys | **Без драйверов (OS API / sysfs)** |
| **Поддержка Linux** | ❌ Отсутствует | ⚠️ Экспериментальная | ** Нативная (hwmon, udev)** |
| **Языки интерфейса** | EN / ZH | RU / EN | **EN, RU, ZH, DE, ES** |
| **Статус** | Вендорский блоат | 📦 В архиве | **⭐ Актуальный релиз** |