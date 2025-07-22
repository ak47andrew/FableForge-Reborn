using Raylib_cs;

namespace Vtt.Widgets;


public class ButtonStyle
{
    // Styles

    // Primary Action (Blue)
    public static readonly ButtonStyle styleBlue = new ButtonStyle(
        new Color(150, 150, 170, 255),  // Disabled
        new Color(65, 105, 225, 255),   // Normal (Royal Blue)
        new Color(100, 149, 237, 255),  // Hovered (Light Sky Blue)
        new Color(25, 25, 112, 255)     // Pressed (Midnight Blue)
    );
    public static readonly ButtonStyle stylePrimary = styleBlue;

    // Success/Confirm (Green)
    public static readonly ButtonStyle styleGreen = new ButtonStyle(
        new Color(170, 200, 170, 255),
        new Color(50, 205, 50, 255),    // LimeGreen
        new Color(144, 238, 144, 255),  // LightGreen
        new Color(0, 100, 0, 255)       // DarkGreen
    );
    public static readonly ButtonStyle styleSuccess = styleGreen;

    // Warning (Orange/Yellow)
    public static readonly ButtonStyle styleOrange = new ButtonStyle(
        new Color(180, 170, 150, 255),
        new Color(255, 140, 0, 255),    // DarkOrange
        new Color(255, 215, 0, 255),    // Gold
        new Color(205, 102, 0, 255)     // DarkOrange (darker)
    );
    public static readonly ButtonStyle styleWarning = styleOrange;

    // Minimalist Light
    public static readonly ButtonStyle styleMinimalLight = new ButtonStyle(
        new Color(230, 230, 230, 180),
        new Color(240, 240, 240, 255),
        new Color(220, 220, 220, 255),
        new Color(200, 200, 200, 255)
    );

    // Dark Theme
    public static readonly ButtonStyle styleDark = new ButtonStyle(
        new Color(40, 40, 40, 200),
        new Color(50, 50, 60, 255),
        new Color(70, 70, 90, 255),
        new Color(30, 30, 40, 255)
    );

    // Purple Accent
    public static readonly ButtonStyle stylePurple = new ButtonStyle(
        new Color(180, 160, 190, 255),
        new Color(147, 112, 219, 255),  // MediumPurple
        new Color(186, 85, 211, 255),   // MediumOrchid
        new Color(75, 0, 130, 255)      // Indigo
    );

    // Glass/Transparent Effect
    public static readonly ButtonStyle styleGlass = new ButtonStyle(
        new Color(100, 100, 100, 80),
        new Color(200, 200, 200, 100),
        new Color(230, 230, 230, 120),
        new Color(150, 150, 150, 150)
    );

    // Red Button Style
    public static readonly ButtonStyle styleRed = new ButtonStyle(
        new Color(160, 148, 145, 255),
        new Color(255, 107, 107, 255),
        new Color(255, 140, 140, 255),
        new Color(210, 77, 77, 255)
    );

    // Fields
    public readonly Color DisabledColor;
    public readonly Color NormalColor;
    public readonly Color HoveredColor;
    public readonly Color PressedColor;

    public ButtonStyle(Color disabledColor, Color normalColor, Color hoveredColor, Color pressedColor)
    {
        DisabledColor = disabledColor;
        NormalColor = normalColor;
        HoveredColor = hoveredColor;
        PressedColor = pressedColor;
    }

    // Methods
    public Color GetButtonColor(ButtonMode mode)
    {
        return mode switch
        {
            ButtonMode.Disabled => DisabledColor,
            ButtonMode.Normal => NormalColor,
            ButtonMode.Hovered => HoveredColor,
            ButtonMode.Pressed => PressedColor,
            _ => Color.White
        };
    }
}