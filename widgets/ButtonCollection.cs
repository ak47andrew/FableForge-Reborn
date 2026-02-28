using System.Numerics;

namespace Vtt.Widgets;

class ButtonCollection : Widget
{
    Button[] buttons;

    public ButtonCollection(Vector2 position, MinimalButton?[][] grid, Vector2? size = null,
                            int horizontalSpacing = Settings.BASE_UI_SIZE, int verticalSpacing = Settings.BASE_UI_SIZE)
    {
        List<Button> buttonsList = new();
        if (size == null)
        {
            size = Vector2.One * Settings.BASE_UI_SIZE;
        }

        for (int i = 0; i < grid.Length; i++)
        {
            for (int j = 0; j < grid[i].Length; j++)
            {
                MinimalButton? entry = grid[i][j];
                if (entry == null)
                {
                    continue;
                }
                var button = entry.ToButton(
                    position + new Vector2(j * (horizontalSpacing + size.Value.X), i * (verticalSpacing + size.Value.Y)),
                    size.Value                    
                );
                buttonsList.Add(button);
            }
        }
        
        buttons = buttonsList.ToArray();
    }

    public override void Draw()
    {
        foreach (var button in buttons)
        {
            button.Draw();
        }
    }

    public override void Update(float deltaTime)
    {
        foreach (var button in buttons)
        {
            button.Update(deltaTime);
        }
    }

    public void HandleMouse(Vector2? mousePosition = null)
    {
        foreach (var button in buttons)
        {
            button.handleMouse(mousePosition);
        }
    }
}
