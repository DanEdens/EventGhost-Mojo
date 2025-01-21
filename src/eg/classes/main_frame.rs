use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Orientation, PopoverMenuBar, Paned};
use gio::{Menu, MenuItem};
use super::{Toolbar, StatusBar, LogCtrl, UIComponent};
use crate::core::Error;

// use glib::Error;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;

/// Represents the main application window for EventGhost.
pub struct MainFrame {
    /// The main GTK application window
    pub(crate) window: ApplicationWindow,
    /// The main menu bar
    pub menu_bar: PopoverMenuBar,
    /// The toolbar
    pub toolbar: Toolbar,
    /// The status bar
    pub status_bar: StatusBar,
    /// The log control
    pub log_ctrl: LogCtrl,
    /// The main container
    pub container: Box,
    /// The paned container for log and tree
    pub paned: Paned,
}

impl MainFrame {
    /// Creates a new MainFrame instance.
    ///
    /// # Arguments
    /// * `app` - The GTK Application instance
    ///
    /// # Returns
    /// A new MainFrame with a configured GTK window
    pub fn new(app: &Application) -> Result<Self, Error> {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("EventGhost")
            .default_width(DEFAULT_WINDOW_WIDTH)
            .default_height(DEFAULT_WINDOW_HEIGHT)
            .build();

        // Create main vertical container
        let container = Box::new(Orientation::Vertical, 0);

        // Create paned container for log and tree
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_wide_handle(true);
        paned.set_position(280); // Default width for log panel

        // Initialize UI components
        let (menu_bar, toolbar, status_bar) = Self::init_ui_components();
        let log_ctrl = LogCtrl::new();

        // Add log control to left pane
        paned.set_start_child(Some(log_ctrl.get_widget()));

        // Add components to container
        container.append(&menu_bar);
        container.append(&toolbar.widget);
        container.append(&paned);
        container.append(&status_bar.widget);

        // Add container to window
        window.set_child(Some(&container));

        // Create MainFrame instance
        let main_frame = MainFrame {
            window,
            menu_bar,
            toolbar,
            status_bar,
            log_ctrl,
            container,
            paned,
        };

        // Set up the menu model
        let menu_model = main_frame.create_menu_model();
        main_frame.menu_bar.set_menu_model(Some(&menu_model));

        Ok(main_frame)
    }

    /// Initialize UI components (menu bar, toolbar, status bar)
    fn init_ui_components() -> (PopoverMenuBar, Toolbar, StatusBar) {
        // Initialize menu bar
        let menu_model = Menu::new();
        let menu_bar = PopoverMenuBar::from_model(Some(&menu_model));

        // Initialize toolbar with all buttons
        let mut toolbar = Toolbar::new();
        Self::init_toolbar_buttons(&mut toolbar);

        // Initialize status bar
        let status_bar = StatusBar::new();

        (menu_bar, toolbar, status_bar)
    }

    /// Initialize toolbar buttons with their handlers
    fn init_toolbar_buttons(toolbar: &mut Toolbar) {
        // File operations
        let new_button = toolbar.add_button("new", "/org/eventghost/images/new.png", "New");
        new_button.connect_clicked(|_| println!("New button clicked"));

        let open_button = toolbar.add_button("open", "/org/eventghost/images/open.png", "Open");
        open_button.connect_clicked(|_| println!("Open button clicked"));

        let save_button = toolbar.add_button("save", "/org/eventghost/images/save.png", "Save");
        save_button.connect_clicked(|_| println!("Save button clicked"));
        save_button.set_sensitive(false);

        toolbar.add_separator();

        // Edit operations
        let cut_button = toolbar.add_button("cut", "/org/eventghost/images/cut.png", "Cut");
        cut_button.connect_clicked(|_| println!("Cut button clicked"));

        let copy_button = toolbar.add_button("copy", "/org/eventghost/images/copy.png", "Copy");
        copy_button.connect_clicked(|_| println!("Copy button clicked"));

        let paste_button = toolbar.add_button("paste", "/org/eventghost/images/paste.png", "Paste");
        paste_button.connect_clicked(|_| println!("Paste button clicked"));

        toolbar.add_separator();

        // Undo/Redo
        let undo_button = toolbar.add_button("undo", "/org/eventghost/images/undo.png", "Undo");
        undo_button.connect_clicked(|_| println!("Undo button clicked"));
        undo_button.set_sensitive(false);

        let redo_button = toolbar.add_button("redo", "/org/eventghost/images/redo.png", "Redo");
        redo_button.connect_clicked(|_| println!("Redo button clicked"));
        redo_button.set_sensitive(false);

        toolbar.add_separator();

        // Add items
        let add_plugin_button = toolbar.add_button("add-plugin", "/org/eventghost/images/plugin.png", "Add Plugin");
        add_plugin_button.connect_clicked(|_| println!("Add plugin button clicked"));

        let add_folder_button = toolbar.add_button("add-folder", "/org/eventghost/images/folder.png", "Add Folder");
        add_folder_button.connect_clicked(|_| println!("Add folder button clicked"));

        let add_macro_button = toolbar.add_button("add-macro", "/org/eventghost/images/macro.png", "Add Macro");
        add_macro_button.connect_clicked(|_| println!("Add macro button clicked"));

        let add_event_button = toolbar.add_button("add-event", "/org/eventghost/images/event.png", "Add Event");
        add_event_button.connect_clicked(|_| println!("Add event button clicked"));

        let add_action_button = toolbar.add_button("add-action", "/org/eventghost/images/action.png", "Add Action");
        add_action_button.connect_clicked(|_| println!("Add action button clicked"));

        toolbar.add_separator();

        // Execute and tree operations
        let execute_button = toolbar.add_button("execute", "/org/eventghost/images/Execute.png", "Execute");
        execute_button.connect_clicked(|_| println!("Execute button clicked"));

        toolbar.add_separator();

        let expand_button = toolbar.add_button("expand", "/org/eventghost/images/expand.png", "Expand");
        expand_button.connect_clicked(|_| println!("Expand button clicked"));

        let collapse_button = toolbar.add_button("collapse", "/org/eventghost/images/collapse.png", "Collapse");
        collapse_button.connect_clicked(|_| println!("Collapse button clicked"));

        let expand_children_button = toolbar.add_button("expand-children", "/org/eventghost/images/expand_children.png", "Expand Children");
        expand_children_button.connect_clicked(|_| println!("Expand children button clicked"));

        let collapse_children_button = toolbar.add_button("collapse-children", "/org/eventghost/images/collapse_children.png", "Collapse Children");
        collapse_children_button.connect_clicked(|_| println!("Collapse children button clicked"));

        let expand_all_button = toolbar.add_button("expand-all", "/org/eventghost/images/expand_all.png", "Expand All");
        expand_all_button.connect_clicked(|_| println!("Expand all button clicked"));

        let collapse_all_button = toolbar.add_button("collapse-all", "/org/eventghost/images/collapse_all.png", "Collapse All");
        collapse_all_button.connect_clicked(|_| println!("Collapse all button clicked"));

        // Set tooltips
        Self::init_toolbar_tooltips(toolbar);
    }

    /// Initialize toolbar tooltips
    fn init_toolbar_tooltips(toolbar: &mut Toolbar) {
        toolbar.set_button_tooltip("new", "New (Ctrl+N)");
        toolbar.set_button_tooltip("open", "Open (Ctrl+O)");
        toolbar.set_button_tooltip("save", "Save (Ctrl+S)");
        toolbar.set_button_tooltip("cut", "Cut (Ctrl+X)");
        toolbar.set_button_tooltip("copy", "Copy (Ctrl+C)");
        toolbar.set_button_tooltip("paste", "Paste (Ctrl+V)");
        toolbar.set_button_tooltip("undo", "Undo (Ctrl+Z)");
        toolbar.set_button_tooltip("redo", "Redo (Ctrl+Y)");
        toolbar.set_button_tooltip("add-plugin", "Add Plugin (Shift+Ctrl+P)");
        toolbar.set_button_tooltip("add-folder", "Add Folder (Shift+Ctrl+N)");
        toolbar.set_button_tooltip("add-macro", "Add Macro (Shift+Ctrl+M)");
        toolbar.set_button_tooltip("add-event", "Add Event (Shift+Ctrl+E)");
        toolbar.set_button_tooltip("add-action", "Add Action (Shift+Ctrl+A)");
        toolbar.set_button_tooltip("execute", "Execute (F5)");
        toolbar.set_button_tooltip("expand", "Expand");
        toolbar.set_button_tooltip("collapse", "Collapse");
        toolbar.set_button_tooltip("expand-children", "Expand Children");
        toolbar.set_button_tooltip("collapse-children", "Collapse Children");
        toolbar.set_button_tooltip("expand-all", "Expand All");
        toolbar.set_button_tooltip("collapse-all", "Collapse All");
    }

    /// Create the menu model for the menu bar
    fn create_menu_model(&self) -> Menu {
        let menu_bar = Menu::new();

        // File Menu
        let file_menu = Menu::new();
        file_menu.append(Some("New"), Some("app.new"));
        file_menu.append(Some("Open"), Some("app.open"));
        file_menu.append(Some("Save"), Some("app.save"));
        file_menu.append(Some("Save As"), Some("app.save-as"));
        
        let separator = Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Options"), Some("app.options"));
        
        let separator = Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Restart"), Some("app.restart"));
        file_menu.append(Some("Restart as Admin"), Some("app.restart-admin"));
        
        let separator = Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Exit"), Some("app.quit"));
        
        let file_item = MenuItem::new(Some("File"), Some("file"));
        file_item.set_submenu(Some(&file_menu));
        menu_bar.append_item(&file_item);

        // Edit Menu
        let edit_menu = Menu::new();
        edit_menu.append(Some("Undo"), Some("app.undo"));
        edit_menu.append(Some("Redo"), Some("app.redo"));
        
        let separator = Menu::new();
        edit_menu.append_section(None, &separator);
        
        edit_menu.append(Some("Cut"), Some("app.cut"));
        edit_menu.append(Some("Copy"), Some("app.copy"));
        edit_menu.append(Some("Python"), Some("app.python"));
        edit_menu.append(Some("Paste"), Some("app.paste"));
        edit_menu.append(Some("Delete"), Some("app.delete"));
        
        let separator = Menu::new();
        edit_menu.append_section(None, &separator);
        
        edit_menu.append(Some("Find"), Some("app.find"));
        edit_menu.append(Some("Find Next"), Some("app.find-next"));
        
        let edit_item = MenuItem::new(Some("Edit"), Some("edit"));
        edit_item.set_submenu(Some(&edit_menu));
        menu_bar.append_item(&edit_item);

        // View Menu
        let view_menu = Menu::new();
        view_menu.append(Some("Show Toolbar"), Some("app.show-toolbar"));
        
        let separator = Menu::new();
        view_menu.append_section(None, &separator);
        
        // Log settings submenu
        let log_menu = Menu::new();
        log_menu.append(Some("Show Time"), Some("app.log-time"));
        log_menu.append(Some("Show Date"), Some("app.log-date"));
        log_menu.append(Some("Indent Log"), Some("app.log-indent"));
        log_menu.append(Some("Clear Log"), Some("app.log-clear"));
        
        let log_item = MenuItem::new(Some("Log Settings"), Some("log"));
        log_item.set_submenu(Some(&log_menu));
        view_menu.append_item(&log_item);
        
        let view_item = MenuItem::new(Some("View"), Some("view"));
        view_item.set_submenu(Some(&view_menu));
        menu_bar.append_item(&view_item);

        // Configuration Menu
        let config_menu = Menu::new();
        config_menu.append(Some("Add Plugin"), Some("app.add-plugin"));
        config_menu.append(Some("Add Folder"), Some("app.add-folder"));
        config_menu.append(Some("Add Macro"), Some("app.add-macro"));
        config_menu.append(Some("Add Event"), Some("app.add-event"));
        config_menu.append(Some("Add Action"), Some("app.add-action"));
        
        let separator = Menu::new();
        config_menu.append_section(None, &separator);
        
        config_menu.append(Some("Configure"), Some("app.configure"));
        config_menu.append(Some("Rename"), Some("app.rename"));
        config_menu.append(Some("Execute"), Some("app.execute"));
        
        let config_item = MenuItem::new(Some("Configuration"), Some("configuration"));
        config_item.set_submenu(Some(&config_menu));
        menu_bar.append_item(&config_item);

        // Help Menu
        let help_menu = Menu::new();
        help_menu.append(Some("Contents"), Some("app.help-contents"));
        
        let separator = Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("Web Homepage"), Some("app.web-homepage"));
        help_menu.append(Some("Web Forum"), Some("app.web-forum"));
        help_menu.append(Some("Web Wiki"), Some("app.web-wiki"));
        
        let separator = Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("Check for Updates"), Some("app.check-update"));
        
        let separator = Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("Python Shell"), Some("app.python-shell"));
        help_menu.append(Some("About"), Some("app.about"));
        
        let help_item = MenuItem::new(Some("Help"), Some("help"));
        help_item.set_submenu(Some(&help_menu));
        menu_bar.append_item(&help_item);

        menu_bar
    }
    
    /// Shows the main application window.
    pub fn show(&self) {
        self.window.show();
    }

    /// Gets the window title
    pub fn get_title(&self) -> Option<String> {
        self.window.title().map(|s| s.to_string())
    }

    /// Gets the default width
    pub fn get_default_width(&self) -> i32 {
        self.window.default_width()
    }

    /// Gets the default height
    pub fn get_default_height(&self) -> i32 {
        self.window.default_height()
    }

    /// Updates the enabled state of toolbar buttons based on document state
    pub fn update_toolbar_state(&mut self, can_save: bool, can_undo: bool, can_redo: bool) {
        self.toolbar.enable_button("save", can_save);
        self.toolbar.enable_button("undo", can_undo);
        self.toolbar.enable_button("redo", can_redo);
    }
    
    /// Updates button tooltips with additional information (like keyboard shortcuts)
    pub fn update_button_tooltips(&mut self) {
        Self::init_toolbar_tooltips(&mut self.toolbar);
    }

    /// Connect menu item actions
    fn connect_menu_actions(&self) {
        let log_ctrl = &self.log_ctrl;
        
        // Log menu actions
        let action = gio::SimpleAction::new("log-time", None);
        action.connect_activate(glib::clone!(@weak log_ctrl => move |_, _| {
            log_ctrl.set_time_logging(!log_ctrl.show_time);
        }));
        self.window.add_action(&action);

        let action = gio::SimpleAction::new("log-date", None);
        action.connect_activate(glib::clone!(@weak log_ctrl => move |_, _| {
            log_ctrl.set_date_logging(!log_ctrl.show_date);
        }));
        self.window.add_action(&action);

        let action = gio::SimpleAction::new("log-indent", None);
        action.connect_activate(glib::clone!(@weak log_ctrl => move |_, _| {
            log_ctrl.set_indent(!log_ctrl.indent);
        }));
        self.window.add_action(&action);

        let action = gio::SimpleAction::new("log-clear", None);
        action.connect_activate(glib::clone!(@weak log_ctrl => move |_, _| {
            log_ctrl.clear();
        }));
        self.window.add_action(&action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main_frame_creation() {
        gtk::init().expect("Failed to initialize GTK");
        
        let app = Application::builder()
            .application_id("org.eventghost.test")
            .build();
            
        let main_frame = MainFrame::new(&app).expect("Failed to create MainFrame");
        assert!(main_frame.toolbar.buttons.len() > 0);
    }
} 