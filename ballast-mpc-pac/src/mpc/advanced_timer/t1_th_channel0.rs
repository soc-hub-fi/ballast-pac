#[doc = "Register `T1_TH_CHANNEL0` reader"]
pub struct R(crate::R<T1_TH_CHANNEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1_TH_CHANNEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1_TH_CHANNEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1_TH_CHANNEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1_TH_CHANNEL0` writer"]
pub struct W(crate::W<T1_TH_CHANNEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1_TH_CHANNEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<T1_TH_CHANNEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1_TH_CHANNEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH` reader - ADV_TIMER1 channel 0 threshold configuration bitfield"]
pub struct TH_R(crate::FieldReader<u16, u16>);
impl TH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TH` writer - ADV_TIMER1 channel 0 threshold configuration bitfield"]
pub struct TH_W<'a> {
    w: &'a mut W,
}
impl<'a> TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "ADV_TIMER1 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "5: `101`"]
    TOGGLE_SET_NEXT = 5,
    #[doc = "1: `1`"]
    TOGGLE_CLEAR_NEXT = 1,
    #[doc = "3: `11`"]
    TOGGLE = 3,
    #[doc = "2: `10`"]
    SET_CLEAR_NEXT = 2,
    #[doc = "0: `0`"]
    SET = 0,
    #[doc = "6: `110`"]
    CLEAR_SET_NEXT = 6,
    #[doc = "4: `100`"]
    CLEAR = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - ADV_TIMER1 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            5 => Some(MODE_A::TOGGLE_SET_NEXT),
            1 => Some(MODE_A::TOGGLE_CLEAR_NEXT),
            3 => Some(MODE_A::TOGGLE),
            2 => Some(MODE_A::SET_CLEAR_NEXT),
            0 => Some(MODE_A::SET),
            6 => Some(MODE_A::CLEAR_SET_NEXT),
            4 => Some(MODE_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE_SET_NEXT`"]
    #[inline(always)]
    pub fn is_toggle_set_next(&self) -> bool {
        **self == MODE_A::TOGGLE_SET_NEXT
    }
    #[doc = "Checks if the value of the field is `TOGGLE_CLEAR_NEXT`"]
    #[inline(always)]
    pub fn is_toggle_clear_next(&self) -> bool {
        **self == MODE_A::TOGGLE_CLEAR_NEXT
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == MODE_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `SET_CLEAR_NEXT`"]
    #[inline(always)]
    pub fn is_set_clear_next(&self) -> bool {
        **self == MODE_A::SET_CLEAR_NEXT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR_SET_NEXT`"]
    #[inline(always)]
    pub fn is_clear_set_next(&self) -> bool {
        **self == MODE_A::CLEAR_SET_NEXT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == MODE_A::CLEAR
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - ADV_TIMER1 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn toggle_set_next(self) -> &'a mut W {
        self.variant(MODE_A::TOGGLE_SET_NEXT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn toggle_clear_next(self) -> &'a mut W {
        self.variant(MODE_A::TOGGLE_CLEAR_NEXT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(MODE_A::TOGGLE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn set_clear_next(self) -> &'a mut W {
        self.variant(MODE_A::SET_CLEAR_NEXT)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(MODE_A::SET)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clear_set_next(self) -> &'a mut W {
        self.variant(MODE_A::CLEAR_SET_NEXT)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MODE_A::CLEAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 16)) | ((value as u32 & 7) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER1 channel 0 threshold configuration bitfield"]
    #[inline(always)]
    pub fn th(&self) -> TH_R {
        TH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - ADV_TIMER1 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER1 channel 0 threshold configuration bitfield"]
    #[inline(always)]
    pub fn th(&mut self) -> TH_W {
        TH_W { w: self }
    }
    #[doc = "Bits 16:18 - ADV_TIMER1 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER1 channel 0 threshold configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1_th_channel0](index.html) module"]
pub struct T1_TH_CHANNEL0_SPEC;
impl crate::RegisterSpec for T1_TH_CHANNEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1_th_channel0::R](R) reader structure"]
impl crate::Readable for T1_TH_CHANNEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1_th_channel0::W](W) writer structure"]
impl crate::Writable for T1_TH_CHANNEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1_TH_CHANNEL0 to value 0"]
impl crate::Resettable for T1_TH_CHANNEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
