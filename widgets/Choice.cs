using System.Numerics;
using Raylib_cs;

namespace Vtt.Widgets;

public class Choice : Widget
{
    public string Label;
    public Texture2D? Texture;
    public Vector2 Position;
    public Vector2 Size;
    public Button? ActionButton;
    public float VSpacing;
    public float HSpacing;

    public Choice(Vector2 position, Vector2 size, string label, Texture2D? texture = null, MinimalButton? button = null)
    {
        Position = position;
        Size = size;
        VSpacing = 1f;
        HSpacing = 3f;

        float spaceLeft = Size.Y - Size.X;
        float buttonHeight = (spaceLeft - VSpacing * 2) / 3 * 2;

        Label = label;
        Texture = texture;
        ActionButton = button == null ? null : button.ToButton(
            new Vector2(
                Position.X + HSpacing,
                Position.Y + Size.X + VSpacing * 2 + (spaceLeft - VSpacing) / 3
            ),
            new Vector2(
                Size.X - HSpacing * 2,
                buttonHeight - VSpacing * 2
            )
        );
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
    }

    public void HandleMouse(Vector2 mouse)
    {
        ActionButton?.handleMouse(mouse);
    }
}