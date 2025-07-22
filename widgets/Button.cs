using System.Numerics;
using Raylib_cs;
using Vtt.Managers;
using static Vtt.Utils.Utils;

namespace Vtt.Widgets;

delegate void OnClickCallback<T>(T classType);

public enum ButtonMode
{
    Disabled,
    Normal,
    Hovered,
    Pressed
}

class Button<T> : Widget
{
    Vector2 Position;
    Vector2 Size;

    ButtonMode mode;
    ButtonMode Mode
    {
        get { return mode; }
        set
        {
            if (mode != value)
            {
                mode = value;
                deltaColor = 0f;
                previousColor = nextColor;
                nextColor = BS.GetButtonColor(mode);
            }
        }
    }


    Rectangle _buttonRect;
    OnClickCallback<T> OnClick;
    T ClassInstance;
    float _scale = 1f;
    Color previousColor;
    Color nextColor;
    float deltaColor = 1f;
    ButtonStyle BS;
    Texture2D? Icon;

    public Button(Vector2 position, Vector2 size, OnClickCallback<T> onClick, T classInstance, ButtonStyle buttonStyle, string icon_token = "")
    {
        Position = position;
        Size = size;
        mode = ButtonMode.Normal;
        _buttonRect = new Rectangle(Position.X, Position.Y, Size.X, Size.Y);
        OnClick = onClick;
        ClassInstance = classInstance;
        BS = buttonStyle;
        nextColor = BS.GetButtonColor(Mode);
        Icon = icon_token == "" ? null : ImageManager.getInstance().GetTexture(icon_token);
    }

    public override void Draw()
    {
        Raylib.DrawRectangleRounded(
            new Rectangle(_buttonRect.Position.X + 3, _buttonRect.Position.Y + 3, _buttonRect.Width * _scale, _buttonRect.Height * _scale),
            0.7f,  // Roundness (0-1)
            9,     // Segments
            new Color(0, 0, 0, 50)
        );
        Raylib.DrawRectangleRounded(
            new Rectangle(_buttonRect.Position.X, _buttonRect.Position.Y, _buttonRect.Width * _scale, _buttonRect.Height * _scale),
            0.7f,  // Roundness (0-1)
            9,     // Segments
            ColorLerp(previousColor, nextColor, deltaColor)
        );
        RenderIcon();
    }

    public void RenderIcon()
    {
        if (Icon == null)
        { return; }
        // Get the button's content area (already scaled)
        Rectangle contentRect = new Rectangle(
            _buttonRect.Position.X,
            _buttonRect.Position.Y,
            _buttonRect.Width * _scale,
            _buttonRect.Height * _scale
        );

        // Set padding percentage (adjust this value as needed)
        const float PADDING_PERCENT = 0.05f; // 5% padding on all sides
        
        // Calculate padding amounts
        float paddingX = contentRect.Width * PADDING_PERCENT;
        float paddingY = contentRect.Height * PADDING_PERCENT;
        
        // Calculate available space for icon (after padding)
        float availableWidth = contentRect.Width - (2 * paddingX);
        float availableHeight = contentRect.Height - (2 * paddingY);

        // Calculate aspect ratios
        float buttonAspect = availableWidth / availableHeight;
        float iconAspect = (float)Icon.Value.Width / Icon.Value.Height;

        // Determine best fit scale factor
        float scaleFactor;
        if (iconAspect > buttonAspect) {
            // Icon is wider than available space
            scaleFactor = availableWidth / Icon.Value.Width;
        } else {
            // Icon is taller than available space
            scaleFactor = availableHeight / Icon.Value.Height;
        }

        // Calculate final dimensions
        float destWidth = Icon.Value.Width * scaleFactor;
        float destHeight = Icon.Value.Height * scaleFactor;

        // Center position calculation
        float destX = contentRect.X + (contentRect.Width - destWidth) / 2;
        float destY = contentRect.Y + (contentRect.Height - destHeight) / 2;

        // Draw icon with calculated dimensions
        Raylib.DrawTexturePro(
            Icon.Value,
            new Rectangle(0, 0, Icon.Value.Width, Icon.Value.Height),
            new Rectangle(destX, destY, destWidth, destHeight),
            Vector2.Zero,
            0f,
            Color.White
        );
    }

    public override void Update(float deltaTime)
    {
        if (Mode == ButtonMode.Disabled) return;

        Vector2 mousePosition = Raylib.GetMousePosition();
        bool isMouseOver = Raylib.CheckCollisionPointRec(mousePosition, _buttonRect);

        switch (Mode)
        {
            case ButtonMode.Normal:
            case ButtonMode.Hovered:
                Mode = isMouseOver ? ButtonMode.Hovered : ButtonMode.Normal;

                if (isMouseOver && Raylib.IsMouseButtonPressed(0))
                {
                    Mode = ButtonMode.Pressed;
                }
                break;

            case ButtonMode.Pressed:
                if (Raylib.IsMouseButtonReleased(0))
                {
                    if (isMouseOver)
                    {
                        Mode = ButtonMode.Hovered;
                        OnClick.Invoke(ClassInstance);
                    }
                    else
                    {
                        Mode = ButtonMode.Normal;
                    }
                }
                break;
        }

        if (Mode == ButtonMode.Hovered && _scale < 1.05f)
            _scale += deltaTime;
        else if (Mode != ButtonMode.Hovered && _scale > 1.0f)
            _scale -= deltaTime;

        deltaColor += deltaTime * 4;
        deltaColor = Math.Min(deltaColor, 1f);
    }

    public void SetActive(bool active)
    {
        if (!active)
        {
            Mode = ButtonMode.Disabled;
        }
        else if (active && Mode == ButtonMode.Disabled)
        {
            Mode = ButtonMode.Normal;
        }
    }
}