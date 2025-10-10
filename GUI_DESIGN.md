# GUI Design System

## Color Palette - Professional Dark Theme

The GUI uses a cohesive dark color scheme inspired by VS Code Dark theme, ensuring excellent readability and professional appearance.

### Base Colors

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Darkest Background** | #1e1e1e | (30, 30, 30) | Main window background |
| **Dark Background** | #252526 | (37, 37, 38) | Panel/section background |
| **Medium Background** | #2d2d30 | (45, 45, 48) | Widget background |
| **Light Background** | #3c3c3c | (60, 60, 60) | Hover states |

### Text Colors

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Primary Text** | #dcdcdc | (220, 220, 220) | Main text, labels |
| **Secondary Text** | #969696 | (150, 150, 150) | Descriptions, hints |
| **Dimmed Text** | #646464 | (100, 100, 100) | Disabled elements |

### Accent Colors

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Primary Blue** | #007acc | (0, 122, 204) | Accent, selected items, buttons |
| **Blue Hover** | #28A0DC | (40, 160, 220) | Button hover states |
| **Blue Active** | #0064B4 | (0, 100, 180) | Button active/pressed |

### Status Colors

| Color | RGB | Usage |
|-------|-----|-------|
| **Success Green** | (80, 180, 100) | Play button, success indicators |
| **Warning Orange** | (220, 140, 60) | Validation warnings |
| **Error Red** | (240, 80, 80) | Error messages |

### Border Colors

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Standard Border** | #3c3c3c | (60, 60, 60) | Section borders, dividers |

## Typography

### Font Sizes

- **Heading**: 22px (Header title)
- **Section Title**: 13px (Section headers)
- **Body**: 13px (Standard text)
- **Secondary**: 12px (Labels, hints)
- **Small**: 11px (Status info)

### Font Weights

- **Strong**: Section titles, button labels
- **Normal**: Body text, labels

## Spacing & Layout

### Margins & Padding

- Window margin: 16px
- Section padding: 16px
- Item spacing: 10px (horizontal), 8px (vertical)
- Button padding: 16px (horizontal), 8px (vertical)

### Border Radius

- Subtle rounded corners: 4-6px
- Consistent across all UI elements

## Component Styles

### Sections

```
Background: #252526 (Dark Background)
Border: 1px solid #3c3c3c
Radius: 6px
Padding: 16px
```

### Buttons

**Primary (Play)**
```
Background: RGB(80, 180, 100)
Size: 160x44px
Text: 14px strong
```

**Secondary (Save MP3)**
```
Background: #007acc (Primary Blue)
Size: 160x44px
Text: 14px strong
```

**Danger (Stop)**
```
Background: RGB(200, 70, 70)
Size: 110x44px
Text: 14px strong
```

**Utility (Browse)**
```
Background: #007acc (Primary Blue)
Size: 95x32px
Text: 13px
```

### Exercise Type Buttons

**Selected**
```
Background: #007acc (Primary Blue)
Size: 100x38px
Text: 13px
```

**Unselected**
```
Background: #2d2d30 (Medium Background)
Size: 100x38px
Text: 13px
```

### Status Panel

**Playing State**
```
Background: RGB(35, 45, 35)
Border: 1.5px solid RGB(80, 180, 100)
Icon: ▶ in green
```

**Generating State**
```
Background: RGB(30, 40, 50)
Border: 1.5px solid #007acc
Icon: ⚙ in blue
```

**Idle State**
```
Background: #252526
Border: 1.5px solid #3c3c3c
Icon: ● in gray
```

## Design Principles

1. **Consistency**: All similar elements use the same colors and spacing
2. **Hierarchy**: Clear visual hierarchy through size and color contrast
3. **Readability**: High contrast between text and background (WCAG AA compliant)
4. **Professionalism**: Subtle, modern aesthetic without distracting elements
5. **State Indication**: Clear visual feedback for all interactive states

## Accessibility

- Text contrast ratio: >7:1 (WCAG AAA)
- Interactive elements: Minimum 44x44px touch target
- Hover states: Clear visual feedback
- Disabled states: Reduced opacity, clear visual distinction
- Error messages: High contrast red with icon

## Window Configuration

- Default size: 680x720px
- Minimum size: 600x650px
- Resizable: Yes
- Scrollable content area: Yes

## Visual Consistency

All UI elements follow the same design language:
- Consistent border radius (4-6px)
- Consistent spacing (multiples of 4px)
- Consistent color usage across components
- Consistent typography scale
- Consistent interaction patterns
