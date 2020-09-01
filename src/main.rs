mod color;
mod color_palette;
mod layout_abstract_syntax;
mod layout_abstract_types;
mod layout_attribute_parser;
mod layout_system;
mod resource_cache;
mod window_dom;

use bevy::{prelude::*, render::pass::ClearColor};

use color_palette::*;
use layout_system::LayoutSystem;

// Function initializing the logging system
fn logger() {
	// Display graphics API errors (requires Vulkan SDK is installed)
	#[cfg(feature = "debug")]
	env_logger::init();
}

// Function creating the layout system components
fn layout(mut commands: Commands) {
	// Main window in the XML layout language
	let mut main_window_layout = LayoutSystem::new();
	main_window_layout.add_window(("window", "main"));

	// The layout system has a single component.
	let components = (main_window_layout,);

	commands.spawn(components);
}

// Function initializing the 2D graphics system
fn graphics(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
	let mild_black = ColorPalette::MildBlack.into_color_linear();
	let mild_white = ColorPalette::MildWhite.into_color_linear();
	let dark_gray = ColorPalette::DarkGray.into_color_linear();

	// Create a new 2D camera for our window's viewport
	commands.spawn(UiCameraComponents::default());

	// Load a sample texture and render it
	// let texture_handle = asset_server.load("textures/grid.png").unwrap();
	commands
		// Create a node for the window
		.spawn(NodeComponents {
			style: Style {
				size: Size {
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
				},
				flex_direction: FlexDirection::Column,
				..Default::default()
			},
			..Default::default()
		})
		.with_children(|document| {
			// Footer
			document.spawn(NodeComponents {
				material: materials.add(Color::rgb(mild_black.r, mild_black.g, mild_black.b).into()),
				style: Style {
					size: Size { width: Val::Percent(100.0), height: Val::Px(24.0), },
					..Default::default()
				},
				..Default::default()
			}).with_children(|footer| {
				footer.spawn(TextComponents {
					text: Text {
						value: String::from("File: 1.8 MB | Memory: 137 MB | Scratch: 0.7/12.3 GB"),
						font: asset_server.load("C:\\Windows\\Fonts\\segoeui.ttf").unwrap(),
						style: TextStyle {
							font_size: 18.0,
							color: Color::rgb(mild_white.r, mild_white.g, mild_white.b).into(),
						},
						..Default::default()
					},
					style: Style {
						padding: Rect { top: Val::Px(0.0), bottom: Val::Px(0.0), left: Val::Px(50.0), right: Val::Px(50.0) },
						..Default::default()
					},
					..Default::default()
				});
				footer.spawn(TextComponents {
					text: Text {
						value: String::from("Box Select Objects | [⇧G] Move Selection | [⇧R] Rotate Selection | [⇧S] Scale Selection"),
						font: asset_server.load("C:\\Windows\\Fonts\\segoeui.ttf").unwrap(),
						style: TextStyle {
							font_size: 18.0,
							color: Color::rgb(mild_white.r, mild_white.g, mild_white.b).into(),
						},
						..Default::default()
					},
					style: Style {
						padding: Rect { top: Val::Px(0.0), bottom: Val::Px(0.0), left: Val::Px(50.0), right: Val::Px(50.0) },
						align_self: AlignSelf::FlexEnd,
						..Default::default()
					},
					..Default::default()
				});
			});
			// Viewport
			document.spawn(ImageComponents {
				// material: materials.add(texture_handle.into()),
				material: materials.add(Color::rgb(dark_gray.r, dark_gray.g, dark_gray.b).into()),
				style: Style {
					size: Size {
						width: Val::Percent(100.0),
						height: Val::Percent(100.0),
					},
					..Default::default()
					},
				..Default::default()
			});
			// Header
			document.spawn(NodeComponents {
				material: materials.add(Color::rgb(mild_black.r, mild_black.g, mild_black.b).into()),
				style: Style {
					size: Size { width: Val::Percent(100.0), height: Val::Px(28.0), },
					..Default::default()
				},
				..Default::default()
			}).with_children(|header| {
				let menus = ["File", "Edit", "Comp", "View", "Help"];
				for menu in menus.iter() {
					header.spawn(TextComponents {
						text: Text {
							value: String::from(*menu),
							font: asset_server.load("C:\\Windows\\Fonts\\segoeui.ttf").unwrap(),
							style: TextStyle {
								font_size: 18.0,
								color: Color::rgb(mild_white.r, mild_white.g, mild_white.b).into(),
							},
							..Default::default()
						},
						style: Style {
							padding: Rect { top: Val::Px(0.0), bottom: Val::Px(0.0), left: Val::Px(50.0), right: Val::Px(50.0) },
							..Default::default()
						},
						..Default::default()
					});
				}
			});
		});
}

fn main() {
	App::build()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(WindowDescriptor {
            title: "Graphite".to_string(),
            ..Default::default()
        })
		// TODO: we might not need all of the default plugins
		.add_default_plugins()
		.add_startup_system(logger.system())
		.add_startup_system(layout.system())
		.add_startup_system(graphics.system())
		.run();
}
