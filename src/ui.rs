use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs, MatchFlag, QFlags, QSize};
use qt_widgets::{
    QApplication, QLineEdit, QMessageBox, QPushButton,
    QWidget,
    QGroupBox, QHBoxLayout, QFileDialog, QVBoxLayout, QListWidget, QProgressDialog
};
use std::rc::Rc;
use crate::files_map;
use std::path::PathBuf;

struct Progress {
    progress_dialog: QBox<QProgressDialog>
}

impl Progress {
    fn new(form: Rc<Form>, target: &String, sources: &Vec<String>) -> Rc<Progress> {
        unsafe {
            let progress_dialog = QProgressDialog::new_1a(&form.window);
            let this = Rc::new(Self {
                progress_dialog
            });
            files_map::start_copy(&target, &sources, true, this.on_total, this.on_progress);
            this
        }
    }

    fn on_progress(self: &Rc<Self>, path: &PathBuf) {
        self.progress_dialog.set_value(self.progress_dialog.value() + 1);
        self.progress_dialog.set_label_text(&qs(path.clone().into_os_string().into_string().unwrap().as_str()));
    }

    fn on_total(self: &Rc<Self>, total: i32) {
        self.progress_dialog.set_maximum(total);
    }
}


struct Form {
    window: QBox<QWidget>,
    browse_target_btn: QBox<QPushButton>,
    target_file_path: QBox<QLineEdit>,
    add_source_btn: QBox<QPushButton>,
    delete_source_btn: QBox<QPushButton>,
    list_source: QBox<QListWidget>,
    start_btn: QBox<QPushButton>
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
            window.set_minimum_size_1a(&QSize::new_2a(650, 450));
            let v_layout = QVBoxLayout::new_1a(&window);

            let source_files_box = QGroupBox::from_q_string(&qs("Dossier(s) source"));
            let source_h_layout = QHBoxLayout::new_1a(&source_files_box);
            let list_source = QListWidget::new_0a();
            let btn_container = QWidget::new_0a();
            btn_container.set_style_sheet(&qs("width: 100px;"));
            source_h_layout.add_widget(&list_source);
            source_h_layout.add_widget(&btn_container);
            let source_btn_v_layout = QVBoxLayout::new_1a(&btn_container);

            let add_source_btn = QPushButton::from_q_string(&qs("Ajouter"));
            let delete_source_btn = QPushButton::from_q_string(&qs("Supprimer"));
            delete_source_btn.set_enabled(false);
            source_btn_v_layout.add_widget(&add_source_btn);
            source_btn_v_layout.add_widget(&delete_source_btn);
            source_btn_v_layout.add_stretch_0a();

            v_layout.add_widget(&source_files_box);

            let target_file_box = QGroupBox::from_q_string(&qs("Dossier cible"));
            let target_files_layout = QHBoxLayout::new_1a(&target_file_box);

            let target_file_path = QLineEdit::new();
            target_files_layout.add_widget(&target_file_path);

            let browse_target_btn = QPushButton::from_q_string(&qs("Parcourir"));
            target_files_layout.add_widget(&browse_target_btn);
            v_layout.add_widget(&target_file_box);

            let start_btn = QPushButton::from_q_string(&qs("Executer"));
            start_btn.set_enabled(false);
            v_layout.add_widget(&start_btn);

            window.show();

            let this = Rc::new(Self {
                window,
                browse_target_btn,
                target_file_path,
                add_source_btn,
                delete_source_btn,
                list_source,
                start_btn
            });
            this.init();
            this
        }
    }

    unsafe fn check_if_we_enable_start_button(self: &Rc<Self>) {
        self.start_btn.set_enabled(self.target_file_path.text().to_std_string().len() > 0 && self.list_source.count() > 0);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_start_button_clicked(self: &Rc<Self>) {
        let target = self.target_file_path.text().to_std_string();
        let mut sources: Vec<String> = Vec::new();
        for i in 0..self.list_source.count() {
            sources.push(self.list_source.item(i).text().to_std_string());
        }
        println!("Target {:?}, sources: {:?}", target, sources);

    }

    #[slot(SlotNoArgs)]
    unsafe fn on_source_list_item_changed(self: &Rc<Self>) {
        let qlist_qlistitem = self.list_source.selected_items().as_raw_ptr().as_ref().unwrap();
        self.delete_source_btn.set_enabled(qlist_qlistitem.count_0a() > 0);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_browse_button_clicked(self: &Rc<Self>) {
        let path = QFileDialog::get_existing_directory_2a(&self.window, &qs("Dossier cible"));
        self.target_file_path.set_text(&path);
        self.check_if_we_enable_start_button();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_add_source_button_clicked(self: &Rc<Self>) {
        let path = QFileDialog::get_existing_directory_2a(&self.window, &qs("Dossier source"));
        let str = path.to_std_string();
        if str == "" {
            return;
        }
        let vec: Vec<&str> = str.split('/').collect();
        let mut is_found = false;
        for i in 0..vec.len() {
            let mut to_search = String::new();
            for j in 0..=i {
                if j > 0 {
                    to_search += "/";
                }
                to_search += vec[j];
            }
            let items_found = self.list_source.find_items(&qs(to_search), QFlags::from(MatchFlag::MatchExactly));
            if items_found.count_0a() > 0 {
                is_found = true;
                break;
            }
        }
        if is_found {
            QMessageBox::warning_q_widget2_q_string(
                &self.window,
                &qs("Attention"),
                &qs("Le chemin spécifié a déjà été ajouté ou se trouve au sein d'un dossier déjà ajouté")
            );
        } else {
            self.list_source.add_item_q_string(&path);
        }
        self.check_if_we_enable_start_button();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_delete_source_button_clicked(self: &Rc<Self>) {
        let current_row = self.list_source.current_row();
        println!("text {:?}", current_row);
        self.list_source.take_item(current_row);
        self.check_if_we_enable_start_button();
    }

    unsafe fn init(self: &Rc<Self>) {
        self.browse_target_btn
            .clicked()
            .connect(&self.slot_on_browse_button_clicked());
        self.add_source_btn
            .clicked()
            .connect(&self.slot_on_add_source_button_clicked());
        self.delete_source_btn
            .clicked()
            .connect(&self.slot_on_delete_source_button_clicked());
        self.start_btn
            .clicked()
            .connect(&self.slot_on_start_button_clicked());
        self.list_source
            .item_selection_changed()
            .connect(&self.slot_on_source_list_item_changed());
    }
}

pub fn window_application() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}