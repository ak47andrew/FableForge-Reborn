using System.Numerics;
using Raylib_cs;
using static Vtt.Utils.Utils;

namespace Vtt.Widgets;

public class Choice : Widget
{
    public string Label;
    public Texture2D? Texture;
    public Vector2 Position;
    public Vector2 Size;
    public Camera2D? Camera;
    public Button<object>? ActionButton;  // FIXME: Replace with actual class type
    public float VSpacing;
    public float HSpacing;

    public Choice(Vector2 position, Vector2 size, string label, Texture2D? texture = null, ButtonStyle? buttonStyle = null, Camera2D? camera = null)
    {
        Position = position;
        Size = size;
        VSpacing = 1f;
        HSpacing = 3f;

        float spaceLeft = Size.Y - Size.X;
        float buttonHeight = (spaceLeft - VSpacing * 2) / 3 * 2;

        Label = label;
        Texture = texture;
        ActionButton = buttonStyle == null ? null : new (
            new Vector2(
                Position.X + HSpacing,
                Position.Y + Size.X + VSpacing * 2 + (spaceLeft - VSpacing) / 3
            ),
            new Vector2(
                Size.X - HSpacing * 2,
                buttonHeight - VSpacing * 2
            ),
            _ => Console.WriteLine("Button clicked"),
            new object(),
            buttonStyle,
            true
        );
        Camera = camera;
    }

    public override void Draw()
    {
        float roundness = 0.3f;
        int segments = 9;

        Raylib.DrawRectangleRounded(
            new Rectangle(Position, Size),
            roundness,
            segments,
            ButtonStyle.styleDark.NormalColor
        );
        if (Texture != null)
        {
            Raylib.DrawTexturePro(
                Texture.Value,
                new Rectangle(0, 0, Texture.Value.Width, Texture.Value.Height),
                new Rectangle(Position, Size.X, Size.X),
                Vector2.Zero,
                0f,
                Color.White
            );
        }

        float spaceLeft = Size.Y - Size.X;
        float textHeight = (spaceLeft - VSpacing) / 3;

        // Text
        Raylib.DrawText(
            Label,
            (int)(Position.X + HSpacing),
            (int)(Position.Y + Size.X + VSpacing),
            (int)textHeight,
            Color.Black
        );

        // Button
        ActionButton?.Draw();
    }

    public override void Update(float deltaTime)
    {
        ActionButton?.Update(deltaTime);
        ActionButton?.handleMouse(Camera == null ? Raylib.GetMousePosition() : GetWorldMousePosition(Camera.Value));
    }
}