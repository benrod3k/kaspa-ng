use crate::imports::*;

pub struct Overview {
    #[allow(dead_code)]
    interop: Interop,
    account : Option<Arc<dyn runtime::Account>>,
}

impl Overview {
    pub fn new(interop: Interop) -> Self {
        Self { interop, account : None }
    }

    pub fn select(&mut self, account : Option<Arc<dyn runtime::Account>>) {
        self.account = account;
    }
}

impl SectionT for Overview {
    fn render(
        &mut self,
        _wallet: &mut Wallet,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {




    }
}