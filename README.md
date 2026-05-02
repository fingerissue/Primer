# 🎨 Primer
> **"The Definitive Foundation for Your Digital Security."**

**Primer**는 학습용 콘텐츠인 `Sketchbook` 시리즈가 아니라, 실제 사용을 목적으로 개발하는 프로젝트입니다. 즉, 내 마음대로 개발하고 내 마음대로 쓸 겁니다.

*(피드백은 언제나 환영입니다!)*

본 프로젝트는 사용자의 마스터 비밀번호를 기반으로 한 **무지식(Zero-Knowledge)** 아키텍처를 지향하며, 어떤 플랫폼에서도 흔들림 없는 보안 환경을 제공하는 것을 최우선 과제로 삼습니다.

---

## 🛡️ Core Principles
- **Production-Ready:** 교육용 코드를 배제하고, 최대한 정교하고 깔끔한 설계를 유지합니다. (진짜 제발요..)
- **Privacy by Default:** 모든 데이터는 기기 내에서 암호화되며, 서버는 오직 암호화된 블롭(Blob)만을 전달합니다.
- **Native Performance:** Rust와 C++의 로우 레벨 제어력을 활용하여 시스템 자원을 최소화하고 반응성을 극대화합니다.

---

## 🏗️ Project Architecture

모듈화된 설계를 통해 각 플랫폼의 특성에 최적화된 인터페이스를 제공하면서도, 보안의 핵심 로직은 단일 코어 엔진에서 관리합니다.

- **`/core`**: **High-Performance Rust Core.** Argon2id 및 AES-256-GCM 표준 준수.
- **`/cli`**: **Primer CLI (`prime`).** 시스템 관리자 수준의 정밀한 금고 제어 도구.
- **`/android`**: **Android Client.** 삼성 갤럭시 환경에 최적화된 Autofill 및 생체 인증 통합.
- **`/windows`**: **Windows Client.** Tauri 기반의 초경량 시스템 트레이 애플리케이션.

> ​💡 Apple 기기는 ㅈㅅ. ~~근데 이미 Apple 생태계에서 지원되지 않나?~~

---

## ⚖️ License & Copyright

- **Code:** MIT License
- **Project Concept & Design:** Copyright 2026. Fingerissue. All rights reserved.
- **Note:** 본 프로젝트는 실사용을 목적으로 제작되었으며, 모든 설계 결정은 보안성을 최우선으로 합니다.
