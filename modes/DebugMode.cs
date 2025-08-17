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
    ButtonCollection<ModeManager> GotoButtons;
    ButtonCollection<object> AllStylesButtons;
    Choice choice;

    public DebugMode()
    {
        string key = ImageManager.getInstance().LoadImage("logo.png");

        GotoButtons = new ButtonCollection<ModeManager>(new Vector2(), [
            [
                new (manager => manager.setMode(new CharacterListMode()), ModeManager.getInstance(), ButtonStyle.styleBlue),
                new (manager => manager.setMode(new CharactersMode()), ModeManager.getInstance(), ButtonStyle.styleOrange),
                new (manager => manager.setMode(new CmdMode()), ModeManager.getInstance(), ButtonStyle.styleRed),
                new (manager => manager.setMode(new MapMode()), ModeManager.getInstance(), ButtonStyle.styleGreen),
            ]
        ]);
        AllStylesButtons = new ButtonCollection<object>(new Vector2(600, 10), [
            [
                new(_ => Console.WriteLine("ButtonStyle.styleBlue"), new object(), ButtonStyle.styleBlue),
                new(_ => Console.WriteLine("ButtonStyle.styleGreen"), new object(), ButtonStyle.styleGreen),
                new(_ => Console.WriteLine("ButtonStyle.styleOrange"), new object(), ButtonStyle.styleOrange),
            ],
            [
                new(_ => Console.WriteLine("ButtonStyle.styleMinimalLight"), new object(), ButtonStyle.styleMinimalLight),
                null,
                new(_ => Console.WriteLine("ButtonStyle.styleDark"), new object(), ButtonStyle.styleDark),
            ],
            [
                new(_ => Console.WriteLine("ButtonStyle.stylePurple"), new object(), ButtonStyle.stylePurple),
                new(_ => Console.WriteLine("ButtonStyle.styleGlass"), new object(), ButtonStyle.styleGlass),
                new(_ => Console.WriteLine("ButtonStyle.styleRed"), new object(), ButtonStyle.styleRed),
            ]
        ], horizontalSpacing: 2, verticalSpacing: 2);
        choice = new(new (1000, 50), new (120, 200), "Pipi", ImageManager.getInstance().GetTexture(key), ButtonStyle.styleGreen, camera);
    }

    public override void Update(float deltaTime)
    {
        base.Update(deltaTime);

        GotoButtons.Update(deltaTime, GetWorldMousePosition(camera));
        AllStylesButtons.Update(deltaTime, GetWorldMousePosition(camera));
        choice.Update(deltaTime);
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
        // Draw origin marker
        Raylib.DrawCircle(0, 0, 10, Color.Red);

        GotoButtons.Draw();
        AllStylesButtons.Draw();
        choice.Draw();
    }
}