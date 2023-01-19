#[doc = "Register `c2c_tx_d` reader"]
pub struct R(crate::R<C2C_TX_D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2C_TX_D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2C_TX_D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2C_TX_D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `c2c_tx_d` writer"]
pub struct W(crate::W<C2C_TX_D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2C_TX_D_SPEC>;
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
impl From<crate::W<C2C_TX_D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2C_TX_D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `drive_strength` reader - "]
pub type DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drive_strength` writer - "]
pub type DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, C2C_TX_D_SPEC, u8, u8, 2, O>;
#[doc = "Field `output_enable` reader - "]
pub type OUTPUT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `output_enable` writer - "]
pub type OUTPUT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, C2C_TX_D_SPEC, bool, O>;
#[doc = "Field `pull_enable` reader - "]
pub type PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `pull_enable` writer - "]
pub type PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, C2C_TX_D_SPEC, bool, O>;
#[doc = "Field `pull_down_up` reader - Pull down 0 Pull up 1"]
pub type PULL_DOWN_UP_R = crate::BitReader<bool>;
#[doc = "Field `pull_down_up` writer - Pull down 0 Pull up 1"]
pub type PULL_DOWN_UP_W<'a, const O: u8> = crate::BitWriter<'a, u16, C2C_TX_D_SPEC, bool, O>;
#[doc = "Field `input_enable` reader - "]
pub type INPUT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `input_enable` writer - "]
pub type INPUT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, C2C_TX_D_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_enable(&self) -> OUTPUT_ENABLE_R {
        OUTPUT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull down 0 Pull up 1"]
    #[inline(always)]
    pub fn pull_down_up(&self) -> PULL_DOWN_UP_R {
        PULL_DOWN_UP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_enable(&self) -> INPUT_ENABLE_R {
        INPUT_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W<0> {
        DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn output_enable(&mut self) -> OUTPUT_ENABLE_W<5> {
        OUTPUT_ENABLE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W<7> {
        PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Pull down 0 Pull up 1"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_up(&mut self) -> PULL_DOWN_UP_W<8> {
        PULL_DOWN_UP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn input_enable(&mut self) -> INPUT_ENABLE_W<9> {
        INPUT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2c_tx_d](index.html) module"]
pub struct C2C_TX_D_SPEC;
impl crate::RegisterSpec for C2C_TX_D_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [c2c_tx_d::R](R) reader structure"]
impl crate::Readable for C2C_TX_D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2c_tx_d::W](W) writer structure"]
impl crate::Writable for C2C_TX_D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets c2c_tx_d to value 0x8008"]
impl crate::Resettable for C2C_TX_D_SPEC {
    const RESET_VALUE: Self::Ux = 0x8008;
}
