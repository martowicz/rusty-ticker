# 🕰️ rusty-ticker - Minutnik w Rust na STM32

Prosty minutnik napisany w Rust dla płytki BlackPill z STM32F411CEU6.

## 🧩 Podzespoły
- **Płytka**: BlackPill v3.1 (STM32F411CEU6, Cortex-M4 100MHz)
- **Wyświetlacz**: LCD 16x2 z I2C (LCM1602)
- **Sterowanie**: Joystick Thumb ST1079
- **Buzzer**: Aktywny (3-5V)

## 📦 Wykorzystane biblioteki
- `rtt_target` - Debugowanie przez RTT 📡
- `cortex_m_rt` - Obsługa Cortex-M ⚙️
- `stm32f4xx_hal` - HAL dla STM32F4 🛠️  
- `lcd_lcm1602_i2c` - Sterownik LCD 📺
- `panic_halt` - Obsługa błędów 🛑

## 🚀 Jak używać?
1. Podłącz układ pod zasilanie przy pomocy złącza `USB-C`
2. Ustaw czas joystickiem:
   - ←/→ : kolejne pola minutnika
   - ↑/↓ : cyfry 0-9
3. Naciśnij joystick aby rozpocząć
4. Po upływie czasu buzzer zacznie piszczeć w krótkich interwałach czasowych
5. Aby wyłaczyć sygnalizację, należy ponownie nacisnąć przycisk