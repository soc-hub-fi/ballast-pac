#[doc = "Register `AU_CFG` reader"]
pub struct R(crate::R<AU_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AU_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AU_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AU_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AU_CFG` writer"]
pub struct W(crate::W<AU_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AU_CFG_SPEC>;
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
impl From<crate::W<AU_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AU_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNED_A {
    #[doc = "0: `0`"]
    NOT_SIGNED = 0,
    #[doc = "1: `1`"]
    SIGNED = 1,
}
impl From<SIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNED` reader - Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed"]
pub struct SIGNED_R(crate::FieldReader<bool>);
impl SIGNED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIGNED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNED_A {
        match self.bits {
            false => SIGNED_A::NOT_SIGNED,
            true => SIGNED_A::SIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SIGNED`"]
    #[inline(always)]
    pub fn is_not_signed(&self) -> bool {
        **self == SIGNED_A::NOT_SIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        **self == SIGNED_A::SIGNED
    }
}
impl core::ops::Deref for SIGNED_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGNED` writer - Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed"]
pub struct SIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGNED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_signed(self) -> &'a mut W {
        self.variant(SIGNED_A::NOT_SIGNED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut W {
        self.variant(SIGNED_A::SIGNED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `BYPASS` reader - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
pub struct BYPASS_R(crate::FieldReader<bool>);
impl BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "6: `110`"]
    AU_MODE_AXA_ACCUMULATION = 6,
    #[doc = "12: `1100`"]
    AU_MODE_AXREG1_ACCUMULATION = 12,
    #[doc = "9: `1001`"]
    AU_MODE_AXREG1PLUSB = 9,
    #[doc = "13: `1101`"]
    AU_MODE_APLUSB = 13,
    #[doc = "7: `111`"]
    AU_MODE_AXAPLUSREG0 = 7,
    #[doc = "8: `1000`"]
    AU_MODE_AXREG1 = 8,
    #[doc = "5: `101`"]
    AU_MODE_AXAMINUSB = 5,
    #[doc = "15: `1111`"]
    AU_MODE_APLUSREG0 = 15,
    #[doc = "11: `1011`"]
    AU_MODE_AXREG1PLUSREG0 = 11,
    #[doc = "4: `100`"]
    AU_MODE_AXAPLUSB = 4,
    #[doc = "3: `11`"]
    AU_MODE_AXA = 3,
    #[doc = "0: `0`"]
    AU_MODE_AXB = 0,
    #[doc = "14: `1110`"]
    AU_MODE_AMINUSB = 14,
    #[doc = "10: `1010`"]
    AU_MODE_AXREG1MINUSB = 10,
    #[doc = "1: `1`"]
    AU_MODE_AXBPLUSREG0 = 1,
    #[doc = "2: `10`"]
    AU_MODE_AXB_ACCUMULATION = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
pub struct MODE_R(crate::FieldReader<u8>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            6 => MODE_A::AU_MODE_AXA_ACCUMULATION,
            12 => MODE_A::AU_MODE_AXREG1_ACCUMULATION,
            9 => MODE_A::AU_MODE_AXREG1PLUSB,
            13 => MODE_A::AU_MODE_APLUSB,
            7 => MODE_A::AU_MODE_AXAPLUSREG0,
            8 => MODE_A::AU_MODE_AXREG1,
            5 => MODE_A::AU_MODE_AXAMINUSB,
            15 => MODE_A::AU_MODE_APLUSREG0,
            11 => MODE_A::AU_MODE_AXREG1PLUSREG0,
            4 => MODE_A::AU_MODE_AXAPLUSB,
            3 => MODE_A::AU_MODE_AXA,
            0 => MODE_A::AU_MODE_AXB,
            14 => MODE_A::AU_MODE_AMINUSB,
            10 => MODE_A::AU_MODE_AXREG1MINUSB,
            1 => MODE_A::AU_MODE_AXBPLUSREG0,
            2 => MODE_A::AU_MODE_AXB_ACCUMULATION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXA_ACCUMULATION`"]
    #[inline(always)]
    pub fn is_au_mode_ax_a_accumulation(&self) -> bool {
        **self == MODE_A::AU_MODE_AXA_ACCUMULATION
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXREG1_ACCUMULATION`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1_accumulation(&self) -> bool {
        **self == MODE_A::AU_MODE_AXREG1_ACCUMULATION
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXREG1PLUSB`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1plus_b(&self) -> bool {
        **self == MODE_A::AU_MODE_AXREG1PLUSB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_APLUSB`"]
    #[inline(always)]
    pub fn is_au_mode_aplus_b(&self) -> bool {
        **self == MODE_A::AU_MODE_APLUSB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXAPLUSREG0`"]
    #[inline(always)]
    pub fn is_au_mode_ax_aplus_reg0(&self) -> bool {
        **self == MODE_A::AU_MODE_AXAPLUSREG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXREG1`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1(&self) -> bool {
        **self == MODE_A::AU_MODE_AXREG1
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXAMINUSB`"]
    #[inline(always)]
    pub fn is_au_mode_ax_aminus_b(&self) -> bool {
        **self == MODE_A::AU_MODE_AXAMINUSB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_APLUSREG0`"]
    #[inline(always)]
    pub fn is_au_mode_aplus_reg0(&self) -> bool {
        **self == MODE_A::AU_MODE_APLUSREG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXREG1PLUSREG0`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1plus_reg0(&self) -> bool {
        **self == MODE_A::AU_MODE_AXREG1PLUSREG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXAPLUSB`"]
    #[inline(always)]
    pub fn is_au_mode_ax_aplus_b(&self) -> bool {
        **self == MODE_A::AU_MODE_AXAPLUSB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXA`"]
    #[inline(always)]
    pub fn is_au_mode_ax_a(&self) -> bool {
        **self == MODE_A::AU_MODE_AXA
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXB`"]
    #[inline(always)]
    pub fn is_au_mode_ax_b(&self) -> bool {
        **self == MODE_A::AU_MODE_AXB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AMINUSB`"]
    #[inline(always)]
    pub fn is_au_mode_aminus_b(&self) -> bool {
        **self == MODE_A::AU_MODE_AMINUSB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXREG1MINUSB`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1minus_b(&self) -> bool {
        **self == MODE_A::AU_MODE_AXREG1MINUSB
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXBPLUSREG0`"]
    #[inline(always)]
    pub fn is_au_mode_ax_bplus_reg0(&self) -> bool {
        **self == MODE_A::AU_MODE_AXBPLUSREG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AXB_ACCUMULATION`"]
    #[inline(always)]
    pub fn is_au_mode_ax_b_accumulation(&self) -> bool {
        **self == MODE_A::AU_MODE_AXB_ACCUMULATION
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn au_mode_ax_a_accumulation(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXA_ACCUMULATION)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1_accumulation(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXREG1_ACCUMULATION)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1plus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXREG1PLUSB)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn au_mode_aplus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_APLUSB)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn au_mode_ax_aplus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXAPLUSREG0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXREG1)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn au_mode_ax_aminus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXAMINUSB)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn au_mode_aplus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_APLUSREG0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1plus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXREG1PLUSREG0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn au_mode_ax_aplus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXAPLUSB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn au_mode_ax_a(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXA)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn au_mode_ax_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXB)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn au_mode_aminus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AMINUSB)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1minus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXREG1MINUSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn au_mode_ax_bplus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXBPLUSREG0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn au_mode_ax_b_accumulation(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AXB_ACCUMULATION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SHIFT` reader - Arithmetic Unit shift window size, (0 – 31)."]
pub struct SHIFT_R(crate::FieldReader<u8>);
impl SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` writer - Arithmetic Unit shift window size, (0 – 31)."]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed"]
    #[inline(always)]
    pub fn signed(&self) -> SIGNED_R {
        SIGNED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Arithmetic Unit shift window size, (0 – 31)."]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed"]
    #[inline(always)]
    pub fn signed(&mut self) -> SIGNED_W {
        SIGNED_W { w: self }
    }
    #[doc = "Bit 1 - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bits 8:11 - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:20 - Arithmetic Unit shift window size, (0 – 31)."]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER arithmetic unit configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [au_cfg](index.html) module"]
pub struct AU_CFG_SPEC;
impl crate::RegisterSpec for AU_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [au_cfg::R](R) reader structure"]
impl crate::Readable for AU_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [au_cfg::W](W) writer structure"]
impl crate::Writable for AU_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AU_CFG to value 0"]
impl crate::Resettable for AU_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
