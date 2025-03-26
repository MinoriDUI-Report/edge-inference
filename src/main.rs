use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

fn main() -> PyResult<()> {
    // Python 인터프리터 초기화
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        // sys.path에 "src" 디렉토리 추가
        let sys = py.import("sys")?;
        let sys_path: &PyList = sys.getattr("path")?.downcast()?;
        sys_path.insert(0, "src")?;

        // Python 버전 확인
        let version = sys.getattr("version")?;
        println!("Python version: {}", version);

        // python_inference.py 모듈 가져오기
        let inference_module = py.import("python_inference")?;

        // inference_dummy 함수 호출: 인자로 5를 넘김
        let result = inference_module.call_method1("inference_dummy", (5,))?;
        let rust_value: i32 = result.extract()?;
        println!("Rust: inference_dummy returned => {}", rust_value);

        // 딕셔너리 기반 인자 전달 예시
        let kwargs = [("x", 10)].into_py_dict(py);
        let result2 = inference_module.call_method("inference_dummy", (), Some(kwargs))?;
        let rust_value2: i32 = result2.extract()?;
        println!("Rust: inference_dummy returned => {}", rust_value2);

        Ok(())
    })
}