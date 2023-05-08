use leptos::*;
use leptos_meta::{Style, StyleProps};
use stylist::Style;
#[component]

pub fn UploadStyles(cx: Scope) -> impl IntoView {
    view! {cx,
        <Style>
            ".uploadLabel {
          justify-content: center;
          font-size: 25px;
          justify-self: center;
          cursor: pointer;
        }

        .display-box {
          position: inherit;
          border-radius: 5px;
          background: #333;
          padding: 10px;
          color: #eeeeee;
          justify-content: center;
        }

        .uploadLabel-small {
          font-size: 15px;
          padding: 5px;
        }

        .submit-button {
          position: inherit;
          border-radius: 3px;
          border: none;
          background: #9a4aff;
          padding: 5px;
          color: #eeeeee;
          justify-content: center;
          cursor: pointer;
        }

        .submit-button-large {
          border-radius: 5px;
          font-size: 25px;
          padding: 10px 40px;
        }

        #thumbnails img:hover {
          filter: brightness(100%);
          box-shadow: 0 0 5px 2px rgba(156, 65, 242, 1);
          transition: filter 0.5s, box-shadow 0.5s;
        }

        #thumbnails img:hover::after {
          content: '';
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          border-radius: 50%;
          box-shadow: 0 0 10px 5px rgba(156, 65, 242, 1);
          z-index: -1;
          animation: glow 1s ease-out infinite;
        }

        @keyframes glow {
          0% {
            box-shadow: 0 0 10px 5px rgba(156, 65, 242, 1);
          }
          100% {
            box-shadow: 0 0 20px 10px rgba(156, 65, 242, 1);
          }
        }
"
        </Style>
    }
}

pub fn drag_styles() -> std::result::Result<styled::Styles, stylist::Error> {
    Style::new(
        r#"
        #drop-area {
          border: 2px dashed #ccc;
          width: 70%;
          height: 70vh;
          display: grid;
          row-gap: 20px;
          column-gap: 10px;
          justify-content: center;
          justify-self: center;
          align-items: center;
          text-align: center;
          border-radius: 20px;
          color: #333;
        }

        #drop-area i {
          justify-self: center;
          text-align: center;
          font-size: 6em;
          margin: auto;
        }

        #result i {
          font-size: 1em;
          color: green;
        }

        #drag-drop-section {
          position: relative;
          height: max(100%, 500px);
          width: 100%;
          justify-content: center;
          display: flex;
        }

        .upload {
          opacity: 0;
          width: 0%;
        }

        #upload-instructions {
          justify-content: center;
          text-align: center;
          height: 100%;
        }

        #upload-instructions div {
          justify-content: center;
          text-align: center;
          color: #eeeeee;
        }

        #upload-instructions > div {
          transition: transform 0.5s ease-in-out;
        }

        body > div {
          justify-content: center;
          text-align: center;
          margin: auto;
        }

        #showcase {
          display: block;
          overflow: auto;
          height: 100%;
        }

        #showcase iframe {
          margin: auto;
          justify-content: center;
          height: 70vh;
          width: 70vw;
          /* position: absolute; */
        }

        /* #thumbnails iframe{ */
        /* height: 100vh; */
        /* width: 20vw; */
        /* position: absolute; */

        /* } */

        .removeBox {
          font-size: 15px;
          text-align: center;
          background: #f63000;
          border: none;
          margin: 15px;
        }

        .progress-container {
          display: flex;
          margin: auto;
          width: 100%;
          height: 20px;
          justify-content: center;
          /* border: 1px solid #ccc; */
        }

        .progress-bar {
          width: 0%;
          height: 100%;
          background-color: #4caf50;
          transition: width 0.2s;
        }

        .progress-value {
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          text-align: center;
          font-size: 12px;
          color: #fff;
        }

        #loading {
            width: 2rem;
            height: 2rem;
            border: 5px solid #f3f3f3;
            border-top: 6px solid #9c41f2;
            border-radius: 100%;
            margin: auto;
            animation: spin 0.5s infinite linear;
        }

        .loading {
            width: "40px !important";
            height: "40px !important";
            transition-duration: 0s;
            border: "5px solid #434b53 !important";
            background: #212529;
            border-top: "6px solid #9c41f2 !important";
            border-radius: 100%;
            margin: auto;
            /* margin: auto; */
            animation: spin 0.5s infinite linear;
        }

        @keyframes spin {
            from {
                transform: rotate(0deg);
            }
            to {
                transform: rotate(360deg);
            }
        }
        "#,
    )
}
