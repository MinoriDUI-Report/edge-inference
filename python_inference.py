import random
import time

def file_inference(video_path: str) -> str:
    """
    더미(demo) 추론 함수.
    영상 파일 경로를 입력받아, 프레임 처리를 흉내내고
    간단한 결과 문자열을 반환.
    """
    print(f"[Python] Received video path: {video_path}")

    simulated_fps = 10
    time_per_frame = 1.0 / simulated_fps
    frame_count = 50

    for i in range(frame_count):
        time.sleep(time_per_frame)

    random_result = random.choice(["Normal", "Drunk", "Uncertain"])
    return f"Dummy Inference Result: {random_result}"