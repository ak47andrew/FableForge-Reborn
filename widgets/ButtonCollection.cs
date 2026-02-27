using System.Numerics;

namespace Vtt.Widgets;

class ButtonCollectionEntry
{
    public Action OnClick;
    public ButtonStyle ButtonStyle;
    public string IconToken;

    public ButtonCollectionEntry(Action onClick, ButtonStyle buttonStyle, string iconToken = "")
    {
        OnClick = onClick;
        ButtonStyle = buttonStyle;
        IconToken = iconToken;
    }
}

class ButtonCollection
{
    Button[] buttons;

    public ButtonCollection(Vector2 position, ButtonCollectionEntry?[][] grid, Vector2? size = null,
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
                ButtonCollectionEntry? entry = grid[i][j];
                if (entry == null)
                {
                    continue;
                }
                var button = new Button(
                    position + new Vector2(j * (horizontalSpacing + size.Value.X), i * (verticalSpacing + size.Value.Y)),
                    size.Value,
                    entry.OnClick,
                    entry.ButtonStyle,
                    icon_token:entry.IconToken
                );
                buttonsList.Add(button);
            }
        }
        
        buttons = buttonsList.ToArray();
    }

    public void Draw()
    {
        foreach (var button in buttons)
        {
            button.Draw();
        }
    }

    public void Update(float deltaTime, Vector2? mousePosition = null)
    {
        foreach (var button in buttons)
        {
            button.Update(deltaTime);
            button.handleMouse(mousePosition);
        }
    }
}
