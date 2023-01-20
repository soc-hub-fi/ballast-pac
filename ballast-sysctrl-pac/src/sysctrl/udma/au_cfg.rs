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
#[doc = "Field `SIGNED` reader - Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed"]
pub type SIGNED_R = crate::BitReader<SIGNED_A>;
#[doc = "Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SIGNED_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SIGNED_A::NOT_SIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == SIGNED_A::SIGNED
    }
}
#[doc = "Field `SIGNED` writer - Arithmetic Unit result signed or not. -1'b0: not signed -1'b1: signed"]
pub type SIGNED_W<'a, const O: u8> = crate::BitWriter<'a, u32, AU_CFG_SPEC, SIGNED_A, O>;
impl<'a, const O: u8> SIGNED_W<'a, O> {
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
}
#[doc = "Field `BYPASS` reader - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AU_CFG_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    AU_MODE_AX_B = 0,
    #[doc = "1: `1`"]
    AU_MODE_AX_BPLUS_REG0 = 1,
    #[doc = "2: `10`"]
    AU_MODE_AX_B_ACCUMULATION = 2,
    #[doc = "3: `11`"]
    AU_MODE_AX_A = 3,
    #[doc = "4: `100`"]
    AU_MODE_AX_APLUS_B = 4,
    #[doc = "5: `101`"]
    AU_MODE_AX_AMINUS_B = 5,
    #[doc = "6: `110`"]
    AU_MODE_AX_A_ACCUMULATION = 6,
    #[doc = "7: `111`"]
    AU_MODE_AX_APLUS_REG0 = 7,
    #[doc = "8: `1000`"]
    AU_MODE_AX_REG1 = 8,
    #[doc = "9: `1001`"]
    AU_MODE_AX_REG1PLUS_B = 9,
    #[doc = "10: `1010`"]
    AU_MODE_AX_REG1MINUS_B = 10,
    #[doc = "11: `1011`"]
    AU_MODE_AX_REG1PLUS_REG0 = 11,
    #[doc = "12: `1100`"]
    AU_MODE_AX_REG1_ACCUMULATION = 12,
    #[doc = "13: `1101`"]
    AU_MODE_APLUS_B = 13,
    #[doc = "14: `1110`"]
    AU_MODE_AMINUS_B = 14,
    #[doc = "15: `1111`"]
    AU_MODE_APLUS_REG0 = 15,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::AU_MODE_AX_B,
            1 => MODE_A::AU_MODE_AX_BPLUS_REG0,
            2 => MODE_A::AU_MODE_AX_B_ACCUMULATION,
            3 => MODE_A::AU_MODE_AX_A,
            4 => MODE_A::AU_MODE_AX_APLUS_B,
            5 => MODE_A::AU_MODE_AX_AMINUS_B,
            6 => MODE_A::AU_MODE_AX_A_ACCUMULATION,
            7 => MODE_A::AU_MODE_AX_APLUS_REG0,
            8 => MODE_A::AU_MODE_AX_REG1,
            9 => MODE_A::AU_MODE_AX_REG1PLUS_B,
            10 => MODE_A::AU_MODE_AX_REG1MINUS_B,
            11 => MODE_A::AU_MODE_AX_REG1PLUS_REG0,
            12 => MODE_A::AU_MODE_AX_REG1_ACCUMULATION,
            13 => MODE_A::AU_MODE_APLUS_B,
            14 => MODE_A::AU_MODE_AMINUS_B,
            15 => MODE_A::AU_MODE_APLUS_REG0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_B`"]
    #[inline(always)]
    pub fn is_au_mode_ax_b(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_BPLUS_REG0`"]
    #[inline(always)]
    pub fn is_au_mode_ax_bplus_reg0(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_BPLUS_REG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_B_ACCUMULATION`"]
    #[inline(always)]
    pub fn is_au_mode_ax_b_accumulation(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_B_ACCUMULATION
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_A`"]
    #[inline(always)]
    pub fn is_au_mode_ax_a(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_A
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_APLUS_B`"]
    #[inline(always)]
    pub fn is_au_mode_ax_aplus_b(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_APLUS_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_AMINUS_B`"]
    #[inline(always)]
    pub fn is_au_mode_ax_aminus_b(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_AMINUS_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_A_ACCUMULATION`"]
    #[inline(always)]
    pub fn is_au_mode_ax_a_accumulation(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_A_ACCUMULATION
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_APLUS_REG0`"]
    #[inline(always)]
    pub fn is_au_mode_ax_aplus_reg0(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_APLUS_REG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_REG1`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_REG1
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_REG1PLUS_B`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1plus_b(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_REG1PLUS_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_REG1MINUS_B`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1minus_b(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_REG1MINUS_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_REG1PLUS_REG0`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1plus_reg0(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_REG1PLUS_REG0
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AX_REG1_ACCUMULATION`"]
    #[inline(always)]
    pub fn is_au_mode_ax_reg1_accumulation(&self) -> bool {
        *self == MODE_A::AU_MODE_AX_REG1_ACCUMULATION
    }
    #[doc = "Checks if the value of the field is `AU_MODE_APLUS_B`"]
    #[inline(always)]
    pub fn is_au_mode_aplus_b(&self) -> bool {
        *self == MODE_A::AU_MODE_APLUS_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_AMINUS_B`"]
    #[inline(always)]
    pub fn is_au_mode_aminus_b(&self) -> bool {
        *self == MODE_A::AU_MODE_AMINUS_B
    }
    #[doc = "Checks if the value of the field is `AU_MODE_APLUS_REG0`"]
    #[inline(always)]
    pub fn is_au_mode_aplus_reg0(&self) -> bool {
        *self == MODE_A::AU_MODE_APLUS_REG0
    }
}
#[doc = "Field `MODE` writer - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AU_CFG_SPEC, u8, MODE_A, 4, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn au_mode_ax_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_B)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn au_mode_ax_bplus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_BPLUS_REG0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn au_mode_ax_b_accumulation(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_B_ACCUMULATION)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn au_mode_ax_a(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_A)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn au_mode_ax_aplus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_APLUS_B)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn au_mode_ax_aminus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_AMINUS_B)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn au_mode_ax_a_accumulation(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_A_ACCUMULATION)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn au_mode_ax_aplus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_APLUS_REG0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_REG1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1plus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_REG1PLUS_B)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1minus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_REG1MINUS_B)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1plus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_REG1PLUS_REG0)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn au_mode_ax_reg1_accumulation(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AX_REG1_ACCUMULATION)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn au_mode_aplus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_APLUS_B)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn au_mode_aminus_b(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_AMINUS_B)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn au_mode_aplus_reg0(self) -> &'a mut W {
        self.variant(MODE_A::AU_MODE_APLUS_REG0)
    }
}
#[doc = "Field `SHIFT` reader - Arithmetic Unit shift window size, (0 – 31)."]
pub type SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHIFT` writer - Arithmetic Unit shift window size, (0 – 31)."]
pub type SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AU_CFG_SPEC, u8, u8, 5, O>;
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
    #[must_use]
    pub fn signed(&mut self) -> SIGNED_W<0> {
        SIGNED_W::new(self)
    }
    #[doc = "Bit 1 - Arithmetic Unit bypass or not. -1'b0: not bypass AU -1'b1: bypass AU"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<1> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 8:11 - Arithmetic Unit mode: -4'b0000: AU_MODE_AxB -4'b0001: AU_MODE_AxB+REG0 -4'b0010: AU_MODE_AxB accumulation -4'b0011: AU_MODE_AxA -4'b0100: AU_MODE_AxA+B -4'b0101: AU_MODE_AxA-B -4'b0110: AU_MODE_AxA accumulation -4'b0111: AU_MODE_AxA+REG0 -4'b1000: AU_MODE_AxREG1 -4'b1001: AU_MODE_AxREG1+B -4'b1010: AU_MODE_AxREG1-B -4'b1011: AU_MODE_AxREG1+REG0 -4'b1100: AU_MODE_AxREG1 accumulation -4'b1101: AU_MODE_A+B -4'b1110: AU_MODE_A-B -4'b1111: AU_MODE_A+REG0"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:20 - Arithmetic Unit shift window size, (0 – 31)."]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> SHIFT_W<16> {
        SHIFT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AU_CFG to value 0"]
impl crate::Resettable for AU_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
