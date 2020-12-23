use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs, MatchFlag, QFlags, QSize, AlignmentFlag, QCoreApplication};
use qt_widgets::{
    QApplication, QLineEdit, QMessageBox, QPushButton,
    QWidget,
    QGroupBox, QHBoxLayout, QFileDialog, QVBoxLayout, QListWidget, QLabel, QProgressBar, QCheckBox
};
use qt_gui::{QIcon};
use std::rc::Rc;
use crate::files_map;

struct Form {
    window: QBox<QWidget>,
    tab_default: QBox<QWidget>,
    browse_target_btn: QBox<QPushButton>,
    target_file_path: QBox<QLineEdit>,
    add_source_btn: QBox<QPushButton>,
    preserver_path_cb: QBox<QCheckBox>,
    delete_source_btn: QBox<QPushButton>,
    list_source: QBox<QListWidget>,
    start_btn: QBox<QPushButton>,
    tab_progress: QBox<QWidget>,
    progress_bar: QBox<QProgressBar>,
    progress_label: QBox<QLabel>,
    section_label: QBox<QLabel>
}

struct ProgressMessage {
    label: String,
    value_nb: i32,
    value_str: String
}

impl std::fmt::Display for ProgressMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(label: {}, nb: {}, str: {})", self.label, self.value_nb, self.value_str)
    }
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
            let main_v_layout = QVBoxLayout::new_1a(&window);

            let tab_default = QWidget::new_0a();
            main_v_layout.add_widget(&tab_default);
            let v_layout = QVBoxLayout::new_1a(&tab_default);

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

            let preserver_path_cb = QCheckBox::from_q_string(&qs("Préserver l'arborescence du dossier source"));
            preserver_path_cb.set_checked(true);
            v_layout.add_widget(&preserver_path_cb);

            let start_btn = QPushButton::from_q_string(&qs("Executer"));
            start_btn.set_enabled(false);
            v_layout.add_widget(&start_btn);

            let tab_progress = QWidget::new_0a();
            main_v_layout.add_widget(&tab_progress);
            let tab_progress_v_layout = QVBoxLayout::new_1a(&tab_progress);

            let progress_bar = QProgressBar::new_0a();
            // progress_bar.set_fixed_width(400);
            progress_bar.set_alignment(QFlags::from(AlignmentFlag::AlignHCenter));
            let section_label = QLabel::from_q_string(&qs("Analyse du dossier cible..."));
            section_label.set_alignment(QFlags::from(AlignmentFlag::AlignHCenter));
            // section_label.set_fixed_width(380);
            let progress_label = QLabel::from_q_string(&qs("Initialisation..."));
            // progress_label.set_alignment(QFlags::from(AlignmentFlag::AlignHCenter));
            // progress_label.set_fixed_width(380);
            tab_progress_v_layout.add_spacing(200);
            tab_progress_v_layout.add_widget(&section_label);
            tab_progress_v_layout.add_widget(&progress_label);
            tab_progress_v_layout.add_widget(&progress_bar);
            tab_progress_v_layout.add_spacing(200);
            tab_progress.set_visible(false);

            window.show();

            let this = Rc::new(Self {
                window,
                browse_target_btn,
                target_file_path,
                add_source_btn,
                delete_source_btn,
                preserver_path_cb,
                list_source,
                start_btn,
                section_label,
                progress_bar,
                progress_label,
                tab_progress,
                tab_default
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

        // let progress_dialog = QDialog::new_1a(&self.window);
        // let v_layout = QVBoxLayout::new_1a(&progress_dialog);
        // let progress_bar = QProgressBar::new_0a();
        // progress_bar.set_fixed_width(400);
        // progress_bar.set_alignment(QFlags::from(AlignmentFlag::AlignHCenter));
        // let section_label = QLabel::from_q_string(&qs("Analyse du dossier cible..."));
        // section_label.set_alignment(QFlags::from(AlignmentFlag::AlignHCenter));
        // section_label.set_fixed_width(380);
        // let progress_label = QLabel::from_q_string(&qs("Initialisation..."));
        // progress_label.set_alignment(QFlags::from(AlignmentFlag::AlignHCenter));
        // progress_label.set_fixed_width(380);
        // v_layout.add_widget(&section_label);
        // v_layout.add_widget(&progress_label);
        // v_layout.add_widget(&progress_bar);

        self.tab_default.set_visible(false);
        self.tab_progress.set_visible(true);
        self.progress_bar.set_value(0);
        self.section_label.set_text(&qs("Analyse du dossier cible..."));
        self.progress_label.set_text(&qs("Initialisation..."));

        QCoreApplication::process_events_0a();


        let is_checked = self.preserver_path_cb.is_checked();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut target_vec: Vec<String>  = Vec::new();
            target_vec.push(target.clone());
            tx.send(ProgressMessage{
                label: String::from("TOTAL_TARGET"),
                value_nb: files_map::get_total_files(&target_vec),
                value_str: String::new()
            }).unwrap();
            tx.send(ProgressMessage{
                label: String::from("TOTAL_SOURCES"),
                value_nb: files_map::get_total_files(&sources),
                value_str: String::new()
            }).unwrap();
            files_map::start_copy(
                &target,
                &sources,
                is_checked,
                &|path| {
                    println!("here, {:?}", path);
                    tx.send(ProgressMessage {
                        label: String::from("PROGRESS_TARGET"),
                        value_nb: 0,
                        value_str: path.clone().into_os_string().into_string().unwrap()
                    });
                },
                &|path| {
                    tx.send(ProgressMessage {
                        label: String::from("PROGRESS_SOURCES"),
                        value_nb: 0,
                        value_str: path.clone().into_os_string().into_string().unwrap()
                    });
                }
            );
        });

        let mut total_target = 0;
        let mut total_sources = 0;
        let mut current = 0;
        let mut target_analyze_done = false;
        for received in rx {
             println!("Got: {}", received);
            if received.label == "TOTAL_TARGET" {
                total_target = received.value_nb;
            } else if received.label == "TOTAL_SOURCES" {
                total_sources = received.value_nb;
            } else if received.label == "PROGRESS_TARGET" {
                current = current + 1;
                let percent = current * 100 / total_target;
                self.progress_bar.set_value(percent);
                self.progress_label.set_text(&qs(received.value_str.as_str()));
                if percent == 100 {
                    QCoreApplication::process_events_0a();
                    current = 0;
                    target_analyze_done = true;
                    self.section_label.set_text(&qs("Parcours des dossiers sources..."));
                    self.progress_bar.set_value(0);
                }
            } else if received.label == "PROGRESS_SOURCES" {
                if !target_analyze_done {
                    self.section_label.set_text(&qs("Parcours des dossiers sources..."));
                    current = 0;
                    target_analyze_done = true;
                }
                current = current + 1;
                let percent = current * 100 / total_sources;
                self.progress_bar.set_value(percent);
                self.progress_label.set_text(&qs(received.value_str.as_str()));
                if percent == 100 {
                    self.tab_progress.set_visible(false);
                    self.tab_default.set_visible(true);
                    QMessageBox::information_q_widget2_q_string(&self.window, &qs("Information"), &qs("Opération effectuée avec succès"));
                }
            }
            QCoreApplication::process_events_0a();
        }
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
        QApplication::set_window_icon(&QIcon::from_q_string(&qs("./sync.ico")));
        QApplication::exec()
    });
}