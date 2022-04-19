use super::{
    super::winapis::{
        direct3d::{cbuffer::CData, model::ModelBuffer, text::*, D3DApplication},
        math::Matrix4x4,
        winapi::*,
        *,
    },
    function::*,
};

pub struct Application {
    winapp: WindowsApplication,
    d3dapp: D3DApplication,
    d3dtxt: D3DTextModule,
    idea: ModelBuffer,
}
impl Application {
    pub fn new(dir: String) -> Result<Self, WErr> {
        let winapp = WindowsApplication::new(
            dir,
            "秘封俱楽部",
            1280,
            720,
            ask_yesno("Start with a fullscreen window?", "question"),
        )?;
        let d3dapp = D3DApplication::new(&winapp, 1280, 720)?;
        let d3dtxt = d3dapp.create_text_module(&winapp)?;
        let idea = create_idea(&d3dapp)?;
        Ok(Self {
            winapp,
            d3dapp,
            d3dtxt,
            idea,
        })
    }
    /// Run the game.
    pub fn run(self) -> Result<(), WErr> {
        let mut cdata = CData {
            mat_scl: Matrix4x4::new_scaling(640.0, 640.0, 1.0),
            mat_rtx: Matrix4x4::new_identity(),
            mat_rty: Matrix4x4::new_identity(),
            mat_rtz: Matrix4x4::new_identity(),
            mat_trs: Matrix4x4::new_translation(0.0, 0.0, 0.0),
            mat_view: Matrix4x4::new_identity(),
            mat_proj: Matrix4x4::new_ortho(-640.0, 640.0, 360.0, -360.0, 0.0, 1.0),
            vec_col: [1.0; 4],
            vec_prm: [0.0; 4],
        };
        let image = self
            .d3dapp
            .create_image_from_file(r"C:\Users\kazuki\OneDrive\touhou\sozai\th_abp\bg_title.png")?;
        let text_desc = DrawingTextDesc {
            text: String::from("秘封俱楽部"),
            font: String::from("メイリオ"),
            size: 64.0,
            rect: [0.0, 1280.0, 0.0, 720.0],
            rgba: [1.0; 4],
            align: TextAlign::Left,
        };
        while !self.winapp.do_event() {
            self.d3dapp.set_rtv();
            self.d3dapp.clear_rtv();
            cdata = self.d3dapp.set_d3dimage(Some(&image), cdata);
            self.d3dapp.set_cdata(&cdata)?;
            self.d3dapp.draw_model(&self.idea)?;
            self.d3dtxt.draw_text(&text_desc)?;
            self.d3dapp.swap()?;
        }
        Ok(())
    }
}
