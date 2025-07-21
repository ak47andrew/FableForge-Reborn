using System.Numerics;
using Raylib_cs;
using static Vtt.Utils.Utils;

namespace Vtt.Widgets;

delegate void OnClickCallback<T>(T classType);

enum ButtonMode
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
                nextColor = GetButtonColor();
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

    public Button(Vector2 position, Vector2 size, OnClickCallback<T> onClick, T classInstance)
    {
        Position = position;
        Size = size;
        Mode = ButtonMode.Normal;
        _buttonRect = new Rectangle(Position.X, Position.Y, Size.X, Size.Y);
        OnClick = onClick;
        ClassInstance = classInstance;
        nextColor = GetButtonColor();
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

    private Color GetButtonColor()
    {
        return Mode switch
        {
            ButtonMode.Disabled => new Color(160, 148, 145, 255),  // Cool slate gray
            ButtonMode.Normal => new Color(255, 107, 107, 255),   // Vibrant blue
            ButtonMode.Hovered => new Color(255, 140, 140, 255),   // Lighter blue
            ButtonMode.Pressed => new Color(210, 77, 77, 255),    // Deep blue
            _ => Color.White
        };
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