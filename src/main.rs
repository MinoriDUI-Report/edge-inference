use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let inference_module = py.import("python_inference")?;

        let video_path = "test_video.mp4";

        let result = inference_module.call_method1("file_inference", (video_path,))?;
        let inference_out: String = result.extract()?;

        println!("[Rust] Python inference output => {}", inference_out);

        Ok(())
    })
}