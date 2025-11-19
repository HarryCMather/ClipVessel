using Avalonia;
using Avalonia.Markup.Xaml;

namespace ClipVessel;

public partial class App : Application
{
    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }
}
