#[doc = "Register `CFG_HI` reader"]
pub struct R(crate::R<CFG_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_HI` writer"]
pub struct W(crate::W<CFG_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_HI_SPEC>;
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
impl From<crate::W<CFG_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Timer enable (starts couting) bit"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Timer enable (starts couting) bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `RST` reader - Timer high reset, cleared after the reset happened"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - Timer high reset, cleared after the reset happened"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `IRQEN` reader - Timer hi interrupt generation on compare match enable"]
pub type IRQEN_R = crate::BitReader<bool>;
#[doc = "Field `IRQEN` writer - Timer hi interrupt generation on compare match enable"]
pub type IRQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `ONES` reader - Timer hi one shot configuration"]
pub type ONES_R = crate::BitReader<bool>;
#[doc = "Field `ONES` writer - Timer hi one shot configuration"]
pub type ONES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `PEN` reader - Timer hi prescaler enable bit"]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - Timer hi prescaler enable bit"]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `CCFG` reader - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub type CCFG_R = crate::BitReader<bool>;
#[doc = "Field `CCFG` writer - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub type CCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_HI_SPEC, bool, O>;
#[doc = "Field `PVAL` reader - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
pub type PVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PVAL` writer - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
pub type PVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_HI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer high reset, cleared after the reset happened"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer hi interrupt generation on compare match enable"]
    #[inline(always)]
    pub fn irqen(&self) -> IRQEN_R {
        IRQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer hi one shot configuration"]
    #[inline(always)]
    pub fn ones(&self) -> ONES_R {
        ONES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer hi prescaler enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    pub fn ccfg(&self) -> CCFG_R {
        CCFG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Timer high reset, cleared after the reset happened"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bit 2 - Timer hi interrupt generation on compare match enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IRQEN_W<2> {
        IRQEN_W::new(self)
    }
    #[doc = "Bit 4 - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - Timer hi one shot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ones(&mut self) -> ONES_W<5> {
        ONES_W::new(self)
    }
    #[doc = "Bit 6 - Timer hi prescaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<6> {
        PEN_W::new(self)
    }
    #[doc = "Bit 7 - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn ccfg(&mut self) -> CCFG_W<7> {
        CCFG_W::new(self)
    }
    #[doc = "Bits 8:15 - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
    #[inline(always)]
    #[must_use]
    pub fn pval(&mut self) -> PVAL_W<8> {
        PVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer High Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_hi](index.html) module"]
pub struct CFG_HI_SPEC;
impl crate::RegisterSpec for CFG_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_hi::R](R) reader structure"]
impl crate::Readable for CFG_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_hi::W](W) writer structure"]
impl crate::Writable for CFG_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_HI to value 0"]
impl crate::Resettable for CFG_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
