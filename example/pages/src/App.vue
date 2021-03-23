<template>
    <div class="main">
        <div class="variants">
            <h3>
                Variants:
                <select v-model="selectedVariant" @change="onVariantsChange">
                    <option value="0">0</option>
                    <option value="1">1</option>
                    <option value="2">2</option>
                    <option value="3">3</option>
                </select>
            </h3>
            <div ref="variants"></div>
        </div>
        <div class="editor">
            <h3>
                Editor:
                <select v-model="mode" @change="onModeChange">
                    <option value="yaml">yaml</option>
                    <option value="toml">toml</option>
                    <option value="json">json</option>
                </select>
            </h3>
            <div ref="editor"></div>
        </div>
        <div class="result">
            <h3>Result:</h3>
            <div ref="result"></div>
        </div>
    </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import * as DefaultContent from './defaultContent';
import _ from 'lodash';
import axios from 'axios';

import CodeMirror from 'codemirror';
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/dracula.css';
import 'codemirror/mode/javascript/javascript.js';
import 'codemirror/mode/yaml/yaml.js';
import 'codemirror/mode/toml/toml.js';

const theme = 'dracula';

type Modes = 'json' | 'yaml' | 'toml';
const modeMap = {
    yaml: 'yaml',
    toml: 'toml',
    json: { name: 'javascript', json: true },
};

@Component({
    components: {},
})
export default class AppPage extends Vue {
    selectedVariant = '0';
    mode: Modes = 'yaml';
    variantsEditor: CodeMirror.Editor;
    resultEditor: CodeMirror.Editor;
    editor: CodeMirror.Editor;

    mounted() {
        this.variantsEditor = CodeMirror(this.$refs['variants'], {
            theme: theme,
            mode: modeMap['json'],
        });
        this.editor = CodeMirror(this.$refs['editor'], {
            theme: theme,
        });
        this.resultEditor = CodeMirror(this.$refs['result'], {
            theme: theme,
            mode: modeMap['json'],
            readOnly: true,
        });
        this.variantsEditor.setValue(JSON.stringify(DefaultContent.variants[0], null, 2));
        this.variantsEditor.on('change', _.debounce(this.convert, 500));
        this.editor.on('change', _.debounce(this.convert, 500));
        this.setDefault(this.mode);
    }

    setDefault(mode: Models) {
        this.setMode(mode);
        let t = '';
        switch (mode) {
            case 'yaml':
                t = DefaultContent.yaml;
                break;
            case 'toml':
                t = DefaultContent.toml;
                break;
            case 'json':
                t = DefaultContent.json;
                break;
            default:
                break;
        }
        this.editor.setValue(t.trim());
    }

    setMode(mode: Modes) {
        this.mode = mode;
        this.editor.setOption('mode', modeMap[mode]);
    }

    onVariantsChange() {
        let v = DefaultContent.variants[parseInt(this.selectedVariant)];
        this.variantsEditor.setValue(JSON.stringify(v, null, 2));
    }

    onModeChange() {
        this.setDefault(this.mode);
    }

    async convert() {
        let text = this.editor.getValue();
        let r = await axios.post('/convert/', {
            type_: this.mode,
            content: text,
            variants: this.variantsEditor.getValue(),
        });
        let t = r.data;
        if (this.mode === 'json') {
            t = JSON.stringify(r.data, null, 2);
        }
        this.resultEditor.setOption('mode', modeMap[this.mode]);
        this.resultEditor.setValue(t);
    }
}
</script>
<style lang="scss">
.main {
    .CodeMirror {
        height: 100%;
    }
}
</style>

<style lang="scss" scoped>
.main {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    // align-items: center;
    // justify-content: center;
    height: 100vh;
    .variants {
        width: 18%;
        > div {
            height: 100%;
        }
    }
    .editor {
        width: 38%;
        > div {
            height: 100%;
        }
    }
    .result {
        width: 38%;
        > div {
            height: 100%;
        }
    }
}
</style>
