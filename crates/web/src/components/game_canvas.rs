use topcoat::{
    Result,
    asset::asset,
    view::{component, view},
};

#[component]
pub async fn game_sandbox() -> Result {
    view! {
        <div
            class="fixed inset-0 w-full h-full pointer-events-none -z-10 overflow-hidden"
        >
            <canvas
                id="bevy_canvas"
                tabindex="0"
                class="w-full h-full block touch-none outline-none"
            ></canvas>
        </div>

        <script
            type="module"
            id="bevy_game"
            data-game=(asset!(
                "../../../../assets/game/pkg/game_client.js", rename : "game"
            ))
            data-wasm=(asset!(
                "../../../../assets/game/pkg/game_client_bg.wasm", rename :
                "game_client_bg"
            ))
        >
            "const s = document.getElementById('bevy_game');"
            "const canvas = document.getElementById('bevy_canvas');"
            "if (canvas) {"
            "    canvas.focus();"
            "    document.addEventListener('pointerdown', function(e) {"
            "        if (!e.target.closest('a, button, input, textarea')) {"
            "            canvas.focus();"
            "        }"
            "    });"
            "    window.addEventListener('keydown', function(e) {"
            "        if (['INPUT', 'TEXTAREA'].includes(document.activeElement?.tagName)) {"
            "            return;"
            "        }"
            "        if (['Space', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.code)) {"
            "            e.preventDefault();"
            "        }"
            "        if (document.activeElement !== canvas) {"
            "            canvas.focus();"
            "            canvas.dispatchEvent(new KeyboardEvent('keydown', e));"
            "        }"
            "    });"
            "    window.addEventListener('keyup', function(e) {"
            "        if (['INPUT', 'TEXTAREA'].includes(document.activeElement?.tagName)) {"
            "            return;"
            "        }"
            "        if (document.activeElement !== canvas) {"
            "            canvas.dispatchEvent(new KeyboardEvent('keyup', e));"
            "        }"
            "    });"
            "}"
            "if (s?.dataset?.game) {"
            "    import(s.dataset.game).then(function(m) {"
            "        return m.default({ module_or_path: s.dataset.wasm }).then(function() {"
            "            m.init();"
            "            canvas?.focus();"
            "        });"
            "    }).catch(function(error) {"
            "        if (!error.message?.includes('Using exceptions for control flow')) {"
            "            console.error('Game launch error:', error);"
            "        }"
            "    });"
            "}"
        </script>
    }
}
