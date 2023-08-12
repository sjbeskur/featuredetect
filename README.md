

### Prerequisites
- **libclang-dev**
    - sudo apt install libclang-dev
- **libstdc++-12-dev**
    - sudo apt install libstdc++-12-dev
- **OpenCV v4.6.0**
    - ```  git clone  https://github.com/opencv/opencv.git  --branch 4.6.0 --depth 1 ```
    - ``` cmake -B target/ -S . ```
    - ``` cd target/ && make -j`nproc` ```
    - ``` sudo make install ```