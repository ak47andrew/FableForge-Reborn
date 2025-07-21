using Raylib_cs;
using System.Numerics;
using Vtt.Managers;
using Vtt.Utils;
using Vtt.Widgets;

namespace Vtt.Modes;

public class MapMode : Mode
{
    Camera2D camera;
    bool isDragging;
    Vector2 dragStartPosition;
    Vector2 dragStartCameraTarget;
    Button<ModeManager> buttonToCharacterSheets1;
    Button<ModeManager> buttonToCharacterSheets2;
    Button<ModeManager> buttonToCharacterSheets3;
    Button<ModeManager> buttonToCharacterSheets4;
    Button<ModeManager> buttonToCharacterSheets5;

    public MapMode()
    {
        camera = new Camera2D();
        camera.Target = Vector2.Zero;
        camera.Offset = new Vector2(Settings.SCREEN.X / 2, Settings.SCREEN.Y / 2); // Center camera
        camera.Rotation = 0.0f;
        camera.Zoom = 1.0f;

        ImageManager.getInstance().LoadImage("bg.png");

        isDragging = false;

        buttonToCharacterSheets1 = new Button<ModeManager>(
            new Vector2(0, 0),
            new Vector2(Settings.BASE_UI_SIZE, Settings.BASE_UI_SIZE),
            manager => Console.WriteLine("LOG!!!"), // manager.setMode(new CharacterListMode()),
            ModeManager.getInstance()
        );
        buttonToCharacterSheets2 = new Button<ModeManager>(
            new Vector2(0, Settings.BASE_UI_SIZE + 3),
            new Vector2(Settings.BASE_UI_SIZE, Settings.BASE_UI_SIZE),
            manager => Console.WriteLine("LOG!!!"), // manager.setMode(new CharacterListMode()),
            ModeManager.getInstance()
        );
        buttonToCharacterSheets3 = new Button<ModeManager>(
            new Vector2(0, Settings.BASE_UI_SIZE * 2 + 6),
            new Vector2(Settings.BASE_UI_SIZE, Settings.BASE_UI_SIZE),
            manager => Console.WriteLine("LOG!!!"), // manager.setMode(new CharacterListMode()),
            ModeManager.getInstance()
        );
        buttonToCharacterSheets4 = new Button<ModeManager>(
            new Vector2(0, Settings.BASE_UI_SIZE * 3 + 9),
            new Vector2(Settings.BASE_UI_SIZE, Settings.BASE_UI_SIZE),
            manager => Console.WriteLine("LOG!!!"), // manager.setMode(new CharacterListMode()),
            ModeManager.getInstance()
        );
        buttonToCharacterSheets5 = new Button<ModeManager>(
            new Vector2(0, Settings.BASE_UI_SIZE * 4 + 12),
            new Vector2(Settings.BASE_UI_SIZE, Settings.BASE_UI_SIZE),
            manager => Console.WriteLine("LOG!!!"), // manager.setMode(new CharacterListMode()),
            ModeManager.getInstance()
        );
    }

    public override void Draw()
    {
        using (new Mode2DContext(camera))
        {
            DrawObjects();
        }
        
        DrawHUD();
    }

    public override void Update(float deltaTime)
    {
        UpdateCameraDrag();
        buttonToCharacterSheets5.Update(deltaTime);
        buttonToCharacterSheets4.Update(deltaTime);
        buttonToCharacterSheets3.Update(deltaTime);
        buttonToCharacterSheets2.Update(deltaTime);
        buttonToCharacterSheets1.Update(deltaTime);
    }

    void UpdateCameraDrag()
    {
        if (Raylib.IsMouseButtonPressed(MouseButton.Right))
        {
            // Start dragging
            isDragging = true;
            dragStartPosition = Raylib.GetMousePosition();
            dragStartCameraTarget = camera.Target;
        }
        else if (Raylib.IsMouseButtonReleased(MouseButton.Right))
        {
            // Stop dragging
            isDragging = false;
        }

        float wheel = Raylib.GetMouseWheelMove();

        if (wheel != 0)
        {
            Vector2 mouseWorldPos = Raylib.GetScreenToWorld2D(Raylib.GetMousePosition(), camera);

            camera.Zoom += wheel * Settings.ZOOM_SPEED;
            camera.Zoom = Math.Clamp(camera.Zoom, Settings.MIN_ZOOM, Settings.MAX_ZOOM);

            Vector2 newMouseWorldPos = Raylib.GetScreenToWorld2D(Raylib.GetMousePosition(), camera);

            camera.Target += mouseWorldPos - newMouseWorldPos;
        }

        if (isDragging)
        {
            // Calculate movement delta in screen space
            Vector2 currentMousePos = Raylib.GetMousePosition();
            Vector2 delta = currentMousePos - dragStartPosition;

            // Convert screen delta to world delta (reverse direction)
            delta /= camera.Zoom; // Adjust for zoom
            camera.Target = dragStartCameraTarget - delta;
        }
    }

    public void DrawHUD()
    {
        if (Settings.DEBUG)
        {
            // Draw fixed GUI elements (screen coordinates)
#pragma warning disable CS0162 // Unreachable code detected
            Raylib.DrawRectangle(10, 10, 200, 140, Color.SkyBlue);
            Raylib.DrawText("Camera Position:", 20, 20, 20, Color.DarkBlue);
            Raylib.DrawText($"X: {camera.Target.X:F2}, Y: {camera.Target.Y:F2}", 20, 50, 20, Color.DarkBlue);
            Raylib.DrawText($"Zoom: {camera.Zoom:F2}x", 20, 80, 20, Color.DarkBlue);
            float wheel = Raylib.GetMouseWheelMove();
            if (wheel > 0) Raylib.DrawText("Scroll: UP", 20, 120, 20, Color.DarkGray);
            else if (wheel < 0) Raylib.DrawText("Scroll: DOWN", 20, 120, 20, Color.Red);
#pragma warning restore CS0162 // Unreachable code detected
        }

        buttonToCharacterSheets5.Draw();
        buttonToCharacterSheets4.Draw();
        buttonToCharacterSheets3.Draw();
        buttonToCharacterSheets2.Draw();
        buttonToCharacterSheets1.Draw();
    }

    public void DrawObjects()
    {
        Texture2D? tmp = ImageManager.getInstance().GetTexture("bg.png");
        if (!tmp.HasValue)
        {
            return;
        }
        Texture2D tex = tmp.Value;

        Raylib.DrawTexture(tex, 0, 0, Color.White);

        for (int x = 0; x <= tex.Width; x += 100)
        {
            Raylib.DrawLine(x, 0, x, tex.Height, Color.DarkGray);
        }
        for (int y = 0; y <= tex.Height; y += 100)
        {
            Raylib.DrawLine(0, y, tex.Width, y, Color.DarkGray);
        }

        // // Draw origin marker
        // Raylib.DrawCircle(0, 0, 10, Color.Red);
    }
}