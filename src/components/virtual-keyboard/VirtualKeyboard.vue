<script setup lang="ts">
import BlackKey from './BlackKey.vue';
import WhiteKey from './WhiteKey.vue';
import { useMidi } from '../../hooks/use-midi';
import { MidiMessage } from '../../types/midi-message';
import { useMouse } from '../../hooks/use-mouse';
import { PropType } from 'vue';

const props = defineProps({
    disabled: {
        type: Boolean as PropType<boolean>,
        default: false,
    },
});

const { sendMessage } = useMidi();
const { isLeftMouseDown } = useMouse();

function handleMouseDown(key: number) {
    if (props.disabled) return;

    sendMessage({
        channel: {
            channel: "channel1",
            message: {
                voice: {
                    noteOn: {
                        note: key,
                        velocity: 127
                    }
                }
            }
        }
    } satisfies MidiMessage);
}

function handleMouseEnter(key: number) {
    if (isLeftMouseDown.value) {
        handleMouseDown(key);
    }
}

function handleMouseUp(key: number) {
    if (props.disabled) return;

    sendMessage({
        channel: {
            channel: "channel1",
            message: {
                voice: {
                    noteOff: {
                        note: key,
                        velocity: 127
                    }
                }
            }
        }
    } satisfies MidiMessage);
}

function handleMouseLeave(key: number) {
    if (isLeftMouseDown.value) {
        handleMouseUp(key);
    }
}
</script>

<template>
    <div class="flex flex-col w-max max-w-full overflow-x-hidden relative"
        :class="{ 'pointer-events-none': disabled, 'mix-blend-overlay': disabled }">
        <div class="w-full h-[2px] bg-red-800/75"></div>
        <div class="flex flex-row overflow-x-auto" :class="{ 'pointer-events-none': disabled }">
            <!-- Sub-contra octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(21)" @mouseenter="handleMouseEnter(21)"
                    @mouseup.left="handleMouseUp(21)" @mouseleave="handleMouseLeave(21)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(22)" @mouseenter="handleMouseEnter(22)"
                    @mouseup.left="handleMouseUp(22)" @mouseleave="handleMouseLeave(22)"
                    class="absolute top-0 left-[18px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(23)" @mouseenter="handleMouseEnter(23)"
                    @mouseup.left="handleMouseUp(23)" @mouseleave="handleMouseLeave(23)"></WhiteKey>
            </div>
            <!-- Contra octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(24)" @mouseenter="handleMouseEnter(24)"
                    @mouseup.left="handleMouseUp(24)" @mouseleave="handleMouseLeave(24)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(25)" @mouseenter="handleMouseEnter(25)"
                    @mouseup.left="handleMouseUp(25)" @mouseleave="handleMouseLeave(25)"
                    class="absolute top-0 left-[14px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(26)" @mouseenter="handleMouseEnter(26)"
                    @mouseup.left="handleMouseUp(26)" @mouseleave="handleMouseLeave(26)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(27)" @mouseenter="handleMouseEnter(27)"
                    @mouseup.left="handleMouseUp(27)" @mouseleave="handleMouseLeave(27)"
                    class="absolute top-0 left-[42px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(28)" @mouseenter="handleMouseEnter(28)"
                    @mouseup.left="handleMouseUp(28)" @mouseleave="handleMouseLeave(28)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(29)" @mouseenter="handleMouseEnter(29)"
                    @mouseup.left="handleMouseUp(29)" @mouseleave="handleMouseLeave(29)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(30)" @mouseenter="handleMouseEnter(30)"
                    @mouseup.left="handleMouseUp(30)" @mouseleave="handleMouseLeave(30)"
                    class="absolute top-0 left-[86px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(31)" @mouseenter="handleMouseEnter(31)"
                    @mouseup.left="handleMouseUp(31)" @mouseleave="handleMouseLeave(31)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(32)" @mouseenter="handleMouseEnter(32)"
                    @mouseup.left="handleMouseUp(32)" @mouseleave="handleMouseLeave(32)"
                    class="absolute top-0 left-[112px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(33)" @mouseenter="handleMouseEnter(33)"
                    @mouseup.left="handleMouseUp(33)" @mouseleave="handleMouseLeave(33)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(34)" @mouseenter="handleMouseEnter(34)"
                    @mouseup.left="handleMouseUp(34)" @mouseleave="handleMouseLeave(34)"
                    class="absolute top-0 left-[138px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(35)" @mouseenter="handleMouseEnter(35)"
                    @mouseup.left="handleMouseUp(35)" @mouseleave="handleMouseLeave(35)"></WhiteKey>
            </div>
            <!-- Great octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(36)" @mouseenter="handleMouseEnter(36)"
                    @mouseup.left="handleMouseUp(36)" @mouseleave="handleMouseLeave(36)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(37)" @mouseenter="handleMouseEnter(37)"
                    @mouseup.left="handleMouseUp(37)" @mouseleave="handleMouseLeave(37)"
                    class="absolute top-0 left-[14px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(38)" @mouseenter="handleMouseEnter(38)"
                    @mouseup.left="handleMouseUp(38)" @mouseleave="handleMouseLeave(38)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(39)" @mouseenter="handleMouseEnter(39)"
                    @mouseup.left="handleMouseUp(39)" @mouseleave="handleMouseLeave(39)"
                    class="absolute top-0 left-[42px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(40)" @mouseenter="handleMouseEnter(40)"
                    @mouseup.left="handleMouseUp(40)" @mouseleave="handleMouseLeave(40)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(41)" @mouseenter="handleMouseEnter(41)"
                    @mouseup.left="handleMouseUp(41)" @mouseleave="handleMouseLeave(41)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(42)" @mouseenter="handleMouseEnter(42)"
                    @mouseup.left="handleMouseUp(42)" @mouseleave="handleMouseLeave(42)"
                    class="absolute top-0 left-[86px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(43)" @mouseenter="handleMouseEnter(43)"
                    @mouseup.left="handleMouseUp(43)" @mouseleave="handleMouseLeave(43)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(44)" @mouseenter="handleMouseEnter(44)"
                    @mouseup.left="handleMouseUp(44)" @mouseleave="handleMouseLeave(44)"
                    class="absolute top-0 left-[112px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(45)" @mouseenter="handleMouseEnter(45)"
                    @mouseup.left="handleMouseUp(45)" @mouseleave="handleMouseLeave(45)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(46)" @mouseenter="handleMouseEnter(46)"
                    @mouseup.left="handleMouseUp(46)" @mouseleave="handleMouseLeave(46)"
                    class="absolute top-0 left-[138px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(47)" @mouseenter="handleMouseEnter(47)"
                    @mouseup.left="handleMouseUp(47)" @mouseleave="handleMouseLeave(47)"></WhiteKey>
            </div>
            <!-- Small octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(48)" @mouseenter="handleMouseEnter(48)"
                    @mouseup.left="handleMouseUp(48)" @mouseleave="handleMouseLeave(48)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(49)" @mouseenter="handleMouseEnter(49)"
                    @mouseup.left="handleMouseUp(49)" @mouseleave="handleMouseLeave(49)"
                    class="absolute top-0 left-[14px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(50)" @mouseenter="handleMouseEnter(50)"
                    @mouseup.left="handleMouseUp(50)" @mouseleave="handleMouseLeave(50)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(51)" @mouseenter="handleMouseEnter(51)"
                    @mouseup.left="handleMouseUp(51)" @mouseleave="handleMouseLeave(51)"
                    class="absolute top-0 left-[42px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(52)" @mouseenter="handleMouseEnter(52)"
                    @mouseup.left="handleMouseUp(52)" @mouseleave="handleMouseLeave(52)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(53)" @mouseenter="handleMouseEnter(53)"
                    @mouseup.left="handleMouseUp(53)" @mouseleave="handleMouseLeave(53)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(54)" @mouseenter="handleMouseEnter(54)"
                    @mouseup.left="handleMouseUp(54)" @mouseleave="handleMouseLeave(54)"
                    class="absolute top-0 left-[86px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(55)" @mouseenter="handleMouseEnter(55)"
                    @mouseup.left="handleMouseUp(55)" @mouseleave="handleMouseLeave(55)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(56)" @mouseenter="handleMouseEnter(56)"
                    @mouseup.left="handleMouseUp(56)" @mouseleave="handleMouseLeave(56)"
                    class="absolute top-0 left-[112px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(57)" @mouseenter="handleMouseEnter(57)"
                    @mouseup.left="handleMouseUp(57)" @mouseleave="handleMouseLeave(57)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(58)" @mouseenter="handleMouseEnter(58)"
                    @mouseup.left="handleMouseUp(58)" @mouseleave="handleMouseLeave(58)"
                    class="absolute top-0 left-[138px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(59)" @mouseenter="handleMouseEnter(59)"
                    @mouseup.left="handleMouseUp(59)" @mouseleave="handleMouseLeave(59)"></WhiteKey>
            </div>
            <!-- One-line octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(60)" @mouseenter="handleMouseEnter(60)"
                    @mouseup.left="handleMouseUp(60)" @mouseleave="handleMouseLeave(60)">C4</WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(61)" @mouseenter="handleMouseEnter(61)"
                    @mouseup.left="handleMouseUp(61)" @mouseleave="handleMouseLeave(61)"
                    class="absolute top-0 left-[14px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(62)" @mouseenter="handleMouseEnter(62)"
                    @mouseup.left="handleMouseUp(62)" @mouseleave="handleMouseLeave(62)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(63)" @mouseenter="handleMouseEnter(63)"
                    @mouseup.left="handleMouseUp(63)" @mouseleave="handleMouseLeave(63)"
                    class="absolute top-0 left-[42px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(64)" @mouseenter="handleMouseEnter(64)"
                    @mouseup.left="handleMouseUp(64)" @mouseleave="handleMouseLeave(64)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(65)" @mouseenter="handleMouseEnter(65)"
                    @mouseup.left="handleMouseUp(65)" @mouseleave="handleMouseLeave(65)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(66)" @mouseenter="handleMouseEnter(66)"
                    @mouseup.left="handleMouseUp(66)" @mouseleave="handleMouseLeave(66)"
                    class="absolute top-0 left-[86px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(67)" @mouseenter="handleMouseEnter(67)"
                    @mouseup.left="handleMouseUp(67)" @mouseleave="handleMouseLeave(67)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(68)" @mouseenter="handleMouseEnter(68)"
                    @mouseup.left="handleMouseUp(68)" @mouseleave="handleMouseLeave(68)"
                    class="absolute top-0 left-[112px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(69)" @mouseenter="handleMouseEnter(69)"
                    @mouseup.left="handleMouseUp(69)" @mouseleave="handleMouseLeave(69)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(70)" @mouseenter="handleMouseEnter(70)"
                    @mouseup.left="handleMouseUp(70)" @mouseleave="handleMouseLeave(70)"
                    class="absolute top-0 left-[138px]">
                </BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(71)" @mouseenter="handleMouseEnter(71)"
                    @mouseup.left="handleMouseUp(71)" @mouseleave="handleMouseLeave(71)"></WhiteKey>
            </div>
            <!-- Two-line octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(72)" @mouseenter="handleMouseEnter(72)"
                    @mouseup.left="handleMouseUp(72)" @mouseleave="handleMouseLeave(72)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(73)" @mouseenter="handleMouseEnter(73)"
                    @mouseup.left="handleMouseUp(73)" @mouseleave="handleMouseLeave(73)"
                    class="absolute top-0 left-[14px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(74)" @mouseenter="handleMouseEnter(74)"
                    @mouseup.left="handleMouseUp(74)" @mouseleave="handleMouseLeave(74)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(75)" @mouseenter="handleMouseEnter(75)"
                    @mouseup.left="handleMouseUp(75)" @mouseleave="handleMouseLeave(75)"
                    class="absolute top-0 left-[42px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(76)" @mouseenter="handleMouseEnter(76)"
                    @mouseup.left="handleMouseUp(76)" @mouseleave="handleMouseLeave(76)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(77)" @mouseenter="handleMouseEnter(77)"
                    @mouseup.left="handleMouseUp(77)" @mouseleave="handleMouseLeave(77)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(78)" @mouseenter="handleMouseEnter(78)"
                    @mouseup.left="handleMouseUp(78)" @mouseleave="handleMouseLeave(78)"
                    class="absolute top-0 left-[86px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(79)" @mouseenter="handleMouseEnter(79)"
                    @mouseup.left="handleMouseUp(79)" @mouseleave="handleMouseLeave(79)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(80)" @mouseenter="handleMouseEnter(80)"
                    @mouseup.left="handleMouseUp(80)" @mouseleave="handleMouseLeave(80)"
                    class="absolute top-0 left-[112px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(81)" @mouseenter="handleMouseEnter(81)"
                    @mouseup.left="handleMouseUp(81)" @mouseleave="handleMouseLeave(81)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(82)" @mouseenter="handleMouseEnter(82)"
                    @mouseup.left="handleMouseUp(82)" @mouseleave="handleMouseLeave(82)"
                    class="absolute top-0 left-[138px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(83)" @mouseenter="handleMouseEnter(83)"
                    @mouseup.left="handleMouseUp(83)" @mouseleave="handleMouseLeave(83)"></WhiteKey>
            </div>
            <!-- Three-line octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(84)" @mouseenter="handleMouseEnter(84)"
                    @mouseup.left="handleMouseUp(84)" @mouseleave="handleMouseLeave(84)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(85)" @mouseenter="handleMouseEnter(85)"
                    @mouseup.left="handleMouseUp(85)" @mouseleave="handleMouseLeave(85)"
                    class="absolute top-0 left-[14px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(86)" @mouseenter="handleMouseEnter(86)"
                    @mouseup.left="handleMouseUp(86)" @mouseleave="handleMouseLeave(86)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(87)" @mouseenter="handleMouseEnter(87)"
                    @mouseup.left="handleMouseUp(87)" @mouseleave="handleMouseLeave(87)"
                    class="absolute top-0 left-[42px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(88)" @mouseenter="handleMouseEnter(88)"
                    @mouseup.left="handleMouseUp(88)" @mouseleave="handleMouseLeave(88)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(89)" @mouseenter="handleMouseEnter(89)"
                    @mouseup.left="handleMouseUp(89)" @mouseleave="handleMouseLeave(89)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(90)" @mouseenter="handleMouseEnter(90)"
                    @mouseup.left="handleMouseUp(90)" @mouseleave="handleMouseLeave(90)"
                    class="absolute top-0 left-[86px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(91)" @mouseenter="handleMouseEnter(91)"
                    @mouseup.left="handleMouseUp(91)" @mouseleave="handleMouseLeave(91)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(92)" @mouseenter="handleMouseEnter(92)"
                    @mouseup.left="handleMouseUp(92)" @mouseleave="handleMouseLeave(92)"
                    class="absolute top-0 left-[112px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(93)" @mouseenter="handleMouseEnter(93)"
                    @mouseup.left="handleMouseUp(93)" @mouseleave="handleMouseLeave(93)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(94)" @mouseenter="handleMouseEnter(94)"
                    @mouseup.left="handleMouseUp(94)" @mouseleave="handleMouseLeave(94)"
                    class="absolute top-0 left-[138px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(95)" @mouseenter="handleMouseEnter(95)"
                    @mouseup.left="handleMouseUp(95)" @mouseleave="handleMouseLeave(95)"></WhiteKey>
            </div>
            <!-- Four-line octave -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(96)" @mouseenter="handleMouseEnter(96)"
                    @mouseup.left="handleMouseUp(96)" @mouseleave="handleMouseLeave(96)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(97)" @mouseenter="handleMouseEnter(97)"
                    @mouseup.left="handleMouseUp(97)" @mouseleave="handleMouseLeave(97)"
                    class="absolute top-0 left-[14px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(98)" @mouseenter="handleMouseEnter(98)"
                    @mouseup.left="handleMouseUp(98)" @mouseleave="handleMouseLeave(98)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(99)" @mouseenter="handleMouseEnter(99)"
                    @mouseup.left="handleMouseUp(99)" @mouseleave="handleMouseLeave(99)"
                    class="absolute top-0 left-[42px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(100)" @mouseenter="handleMouseEnter(100)"
                    @mouseup.left="handleMouseUp(100)" @mouseleave="handleMouseLeave(100)"></WhiteKey>
                <WhiteKey @mousedown.left="handleMouseDown(101)" @mouseenter="handleMouseEnter(101)"
                    @mouseup.left="handleMouseUp(101)" @mouseleave="handleMouseLeave(101)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(102)" @mouseenter="handleMouseEnter(102)"
                    @mouseup.left="handleMouseUp(102)" @mouseleave="handleMouseLeave(102)"
                    class="absolute top-0 left-[86px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(103)" @mouseenter="handleMouseEnter(103)"
                    @mouseup.left="handleMouseUp(103)" @mouseleave="handleMouseLeave(103)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(104)" @mouseenter="handleMouseEnter(104)"
                    @mouseup.left="handleMouseUp(104)" @mouseleave="handleMouseLeave(104)"
                    class="absolute top-0 left-[112px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(105)" @mouseenter="handleMouseEnter(105)"
                    @mouseup.left="handleMouseUp(105)" @mouseleave="handleMouseLeave(105)"></WhiteKey>
                <BlackKey @mousedown.left="handleMouseDown(106)" @mouseenter="handleMouseEnter(106)"
                    @mouseup.left="handleMouseUp(106)" @mouseleave="handleMouseLeave(106)"
                    class="absolute top-0 left-[138px]"></BlackKey>
                <WhiteKey @mousedown.left="handleMouseDown(107)" @mouseenter="handleMouseEnter(107)"
                    @mouseup.left="handleMouseUp(107)" @mouseleave="handleMouseLeave(107)"></WhiteKey>
            </div>
            <!-- C8 -->
            <div class="flex flex-row relative">
                <WhiteKey @mousedown.left="handleMouseDown(108)" @mouseenter="handleMouseEnter(108)"
                    @mouseup.left="handleMouseUp(108)" @mouselease="handleMouseLeave(108)"></WhiteKey>
            </div>
        </div>
    </div>
</template>