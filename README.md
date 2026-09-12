<div align="center">

# IDC-Lite

**Legacy C# / .NET utility for ID-COOLING FX series liquid cooler LCD displays (Windows & Linux)**

[![.NET 8.0](https://img.shields.io/badge/.NET-8.0-512BD4?style=for-the-badge&logo=dotnet&logoColor=white)](https://dotnet.microsoft.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-blue)](README.md)

</div>

---

## Сравнение с оригинальным софтом

| Параметр | Оригинал ID-COOLING (Electron) | IDC-Lite (C# / .NET) |
| :--- | :---: | :---: |
| **Потребление ОЗУ** | ~180 – 250 МБ | ~50 – 80 МБ |
| **Размер бинарника** | 150+ МБ | ~25 МБ |
| **Драйверы ядра** | Закрытый Ring0 драйвер | WinRing0.sys |
| **Поддержка Linux** | Отсутствует | Экспериментальная / костыльная |
| **Языки интерфейса** | EN / ZH | RU / EN / ZH |

---

## Структура репозитория

```text
idc-lite/
├── deb_build/                   # Шаблоны сборки .deb пакетов
├── idc-daemon/                  # Устаревший фоновый демон для Linux
├── idc-lite/                    # Устаревшее десктопное приложение для Windows (WPF)
└── README.md
