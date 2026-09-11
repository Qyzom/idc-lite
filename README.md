<div align="center">

# IDC-Lite

**Legacy C# / .NET utility for ID-COOLING FX series liquid cooler LCD displays (Windows & Linux)**

[![.NET 8.0](https://img.shields.io/badge/.NET-8.0-512BD4?style=for-the-badge&logo=dotnet&logoColor=white)](https://dotnet.microsoft.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-blue)](README.md)

<br />

> ### ⚠️ DEPRECATED & DISCONTINUED IN FAVOR OF [RustCooling](https://github.com/Qyzom/RustCooling)
> 
> This legacy C# (.NET / WPF) implementation is deprecated, unstable, and resource-heavy. It relies on closed-source kernel drivers (`WinRing0`), suffers from runtime overhead, memory fragmentation, and unreliable Linux operation.
> 
> The project has been completely rewritten from scratch in pure **Rust** as a high-performance CLI utility and background daemon: **[RustCooling](https://github.com/Qyzom/RustCooling)**.
> 
> **It is strongly recommended to use [RustCooling](https://github.com/Qyzom/RustCooling) instead!**
> 
> ---
> 
> Данная реализация на C# (.NET / WPF) устарела и признана неэффективной. Она содержит высокий оверхед рантайма, зависит от закрытых драйверов ядра WinRing0 и нестабильна под Linux.
> 
> Проект был полностью переписан с нуля на **Rust** в виде ультралегкой консольной утилиты и фонового демона: **[RustCooling](https://github.com/Qyzom/RustCooling)**.
> 
> **Настоятельно рекомендуется переходить на [RustCooling](https://github.com/Qyzom/RustCooling)!**

</div>

---

## Сравнение с оригинальным софтом

| Параметр | Оригинал ID-COOLING (Electron) | IDC-Lite (C# / .NET) | [RustCooling](https://github.com/Qyzom/RustCooling) (Rust) |
| :--- | :---: | :---: | :---: |
| **Потребление ОЗУ** | ~180 – 250 МБ | ~50 – 80 МБ | **< 2 МБ** |
| **Размер бинарника** | 150+ МБ | ~25 МБ | **~800 КБ** |
| **Драйверы ядра** | Закрытый Ring0 драйвер | WinRing0.sys | **Не требуются (Zero drivers)** |
| **Поддержка Linux** | Отсутствует | Экспериментальная / костыльная | **Нативная (CLI + systemd)** |
| **Языки интерфейса** | EN / ZH | RU / EN / ZH | **EN / RU / ZH** |

---

## Структура репозитория
```text
idc-lite/
├── deb_build/                   # Шаблоны сборки .deb пакетов
├── idc-daemon/                  # Устаревший фоновый демон для Linux
├── idc-lite/                    # Устаревшее десктопное приложение для Windows (WPF)
└── README.md
```

## Сборка из исходников

### Windows (WPF)
```bash
git clone https://github.com/Qyzom/idc-lite.git
cd idc-lite/idc-lite
dotnet publish -c Release -r win-x64 --self-contained
```

### Linux (Daemon)
```bash
cd idc-lite/idc-daemon
dotnet publish -c Release -r linux-x64 --self-contained
```

---

## Лицензия

MIT License. См. [LICENSE](LICENSE).
