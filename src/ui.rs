use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs};
use qt_widgets::{
    QAction, QApplication, QLineEdit, QMenu, QMessageBox, QPushButton, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget, SlotOfQPoint, SlotOfQTableWidgetItemQTableWidgetItem,
    QGroupBox, QHBoxLayout, QFileDialog, q_file_dialog
};
use std::rc::Rc;

struct Form {
    window: QBox<QWidget>,
    line_edit: QBox<QLineEdit>,
    button: QBox<QPushButton>,
    table: QBox<QTableWidget>,
}

impl StaticUpcast<QObject> for Form {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.window.as_ptr().static_upcast()
    }
}

impl Form {
    fn new() -> Rc<Form> {
        unsafe {
            let window = QWidget::new_0a();
            let v_layout = QVBoxLayout::new_1a(&window);

            let source_files_box = QGroupBox::from_q_string(&qs("Dossier(s) source"));
            v_layout.add_widget(&source_files_box);

            let target_file_box = QGroupBox::from_q_string(&qs("Dossier cible"));
            let target_files_layout = QHBoxLayout::new_1a(&target_file_box);

            let line_edit = QLineEdit::new();
            target_files_layout.add_widget(&line_edit);

            let browse = QPushButton::from_q_string(&qs("Parcourir"));
            target_files_layout.add_widget(&browse);
            v_layout.add_widget(&source_files_box);

            //file_box.set_layout(&v_layout);
            //let txt = QFileDialog::get_existing_directory_2a(&window, &qs("Dossier cible"));
            println!("{:?}", txt);
            v_layout.add_widget(&target_file_box);



            let button = QPushButton::from_q_string(&qs("Start"));
            button.set_enabled(false);
            v_layout.add_widget(&button);

            let table = QTableWidget::new_0a();
            table.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
            table.set_row_count(2);
            table.set_column_count(1);

            let item1 = QTableWidgetItem::new().into_ptr();
            item1.set_text(&qs("Item 1"));
            table.set_item(0, 0, item1);

            let item2 = QTableWidgetItem::new().into_ptr();
            item2.set_text(&qs("Item 2"));
            table.set_item(1, 0, item2);

            v_layout.insert_widget_2a(0, &table);

            window.show();

            let this = Rc::new(Self {
                window,
                button,
                line_edit,
                table,
            });
            this.init();
            this
        }
    }

    unsafe fn link(self: &Rc<Self>, text_box: Ptr<QLineEdit>, button: Ptr<QPushButton>) {
        button.clicked().connect(&self.slot_on_directory_selected(text_box));
    }

    #[slot(SlotOfDirectorySelected)]
    unsafe fn on_directory_selected(self: &Rc<Self>, text_box: Ptr<QLineEdit>) {
        let txt = QFileDialog::get_existing_directory_2a(&self.window, &qs("Dossier cible"));
        println!("{}", txt.to_std_string());
    }


    unsafe fn init(self: &Rc<Self>) {
        self.button
            .clicked()
            .connect(&self.slot_on_button_clicked());
        self.line_edit
            .text_edited()
            .connect(&self.slot_on_line_edit_text_edited());
        self.table
            .current_item_changed()
            .connect(&self.slot_on_table_current_item_changed());
        self.table
            .custom_context_menu_requested()
            .connect(&self.slot_on_table_context_menu_requested());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_line_edit_text_edited(self: &Rc<Self>) {
        self.button.set_enabled(!self.line_edit.text().is_empty());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_button_clicked(self: &Rc<Self>) {
        let text = self.line_edit.text();
        QMessageBox::information_q_widget2_q_string(
            &self.window,
            &qs("Example"),
            &qs("Text: \"%1\". Congratulations!").arg_q_string(&text),
        );
    }

    #[slot(SlotOfQTableWidgetItemQTableWidgetItem)]
    unsafe fn on_table_current_item_changed(
        self: &Rc<Self>,
        current: Ptr<QTableWidgetItem>,
        previous: Ptr<QTableWidgetItem>,
    ) {
        if !previous.is_null() {
            let font = previous.font();
            font.set_bold(false);
            previous.set_font(&font);
        }
        if !current.is_null() {
            let font = current.font();
            font.set_bold(true);
            current.set_font(&font);
        }
    }

    #[slot(SlotOfQPoint)]
    unsafe fn on_table_context_menu_requested(self: &Rc<Self>, pos: Ref<QPoint>) {
        let global_pos = self.table.viewport().map_to_global(pos);
        let menu = QMenu::new();
        menu.add_action_q_string(&qs("Fake action 1"));
        menu.add_action_q_string(&qs("Fake action 2"));

        let action3 = QAction::from_q_string(&qs("Real action"));
        menu.add_action(&action3);

        let action = menu.exec_1a_mut(&global_pos);
        let message = if action.is_null() {
            "No action selected!".to_string()
        } else if action.as_raw_ptr() == action3.as_raw_ptr() {
            "Real action selected!".to_string()
        } else {
            format!("Fake action selected ({})", action.text().to_std_string())
        };
        QMessageBox::information_q_widget2_q_string(&self.window, &qs("Example"), &qs(message));
    }
}

pub fn window_application() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}