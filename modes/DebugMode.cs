using Raylib_cs;
using System.Numerics;
using Vtt.Managers;
using Vtt.Widgets;
using static Vtt.Utils.Utils;

namespace Vtt.Modes;

public class DebugMode : DragableMode
{
    int MaxWidthDebugWindow = 0;
    int MaxHeightDebugWindow = 0;

    // Buttons
    Button<ModeManager> buttonToCharacterList;
    Button<ModeManager> buttonToCharacter;
    Button<ModeManager> buttonToCmd;
    Button<ModeManager> buttonToMap;

    public DebugMode()
    {
        buttonToCharacterList = new(
            new Vector2(0, 0),
            Vector2.One * Settings.BASE_UI_SIZE,
            manager => manager.setMode(new CharacterListMode()),
            ModeManager.getInstance(),
            ButtonStyle.styleBlue
        );
        buttonToCharacter = new(
            new Vector2(Settings.BASE_UI_SIZE * 2, 0),
            Vector2.One * Settings.BASE_UI_SIZE,
            manager => manager.setMode(new CharactersMode()),
            ModeManager.getInstance(),
            ButtonStyle.styleOrange
        );
        buttonToCmd = new(
            new Vector2(Settings.BASE_UI_SIZE * 4, 0),
            Vector2.One * Settings.BASE_UI_SIZE,
            manager => manager.setMode(new CmdMode()),
            ModeManager.getInstance(),
            ButtonStyle.styleRed
        );
        buttonToMap = new(
            new Vector2(Settings.BASE_UI_SIZE * 6, 0),
            Vector2.One * Settings.BASE_UI_SIZE,
            manager => manager.setMode(new MapMode()),
            ModeManager.getInstance(),
            ButtonStyle.styleGreen
        );
    }

    public override void Update(float deltaTime)
    {
        base.Update(deltaTime);

        buttonToCharacterList.Update(deltaTime);
        buttonToCharacter.Update(deltaTime);
        buttonToCmd.Update(deltaTime);
        buttonToMap.Update(deltaTime);

        buttonToCharacterList.handleMouse(GetWorldMousePosition(camera));
        buttonToCharacter.handleMouse(GetWorldMousePosition(camera));
        buttonToCmd.handleMouse(GetWorldMousePosition(camera));
        buttonToMap.handleMouse(GetWorldMousePosition(camera));
    }

    public override void DrawHUD()
    {
        float wheel = Raylib.GetMouseWheelMove();
        Vector2 mouseWorldPos = GetWorldMousePosition(camera);

        List<string> strings = new()
        {
            "Camera Position:",
            $"X: {camera.Target.X:F2}, Y: {camera.Target.Y:F2}",
            "Mouse Position (Screen):",
            $"X: {Raylib.GetMouseX()}, Y: {Raylib.GetMouseY()}",
            "Mouse Position (World):",
            $"X: {mouseWorldPos.X}, Y: {mouseWorldPos.Y}",
            $"Zoom: {camera.Zoom:F2}x",
        };
        if (wheel > 0) strings.Add("Scroll: UP");
        else if (wheel < 0) strings.Add("Scroll: DOWN");
        strings.Add($"FPS: {Raylib.GetFPS()}");
        DrawDebugText(strings);
    }

    void DrawDebugText(List<string> texts)
    {
        MaxWidthDebugWindow = Math.Max(texts.Select(text => Raylib.MeasureText(text, 22)).Max(), MaxWidthDebugWindow);
        MaxHeightDebugWindow = Math.Max((texts.Count + 1) * 20, MaxHeightDebugWindow);
        Raylib.DrawRectangle(10, 10, MaxWidthDebugWindow, MaxHeightDebugWindow, new Color(
            102, 191, 255, 100
        ));

        for (int i = 0; i < texts.Count; i++)
        {
            Raylib.DrawText(texts[i], 20, (i + 1) * 20, 20, Color.White);
        }
    }

    public override void DrawObjects()
    {
        buttonToCharacterList.Draw();
        buttonToCharacter.Draw();
        buttonToCmd.Draw();
        buttonToMap.Draw();

        // Draw origin marker
        Raylib.DrawCircle(0, 0, 10, Color.Red);
    }
}