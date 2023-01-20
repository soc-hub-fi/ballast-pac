#[doc = "Register `FCFETCH` writer"]
pub struct W(crate::W<FCFETCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFETCH_SPEC>;
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
impl From<crate::W<FCFETCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFETCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCFETCH` writer - Enable Fetch, 1 to enable core execution"]
pub type FCFETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFETCH_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Enable Fetch, 1 to enable core execution"]
    #[inline(always)]
    #[must_use]
    pub fn fcfetch(&mut self) -> FCFETCH_W<0> {
        FCFETCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the value of the fetch enable signal of the core.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfetch](index.html) module"]
pub struct FCFETCH_SPEC;
impl crate::RegisterSpec for FCFETCH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcfetch::W](W) writer structure"]
impl crate::Writable for FCFETCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFETCH to value 0x01"]
impl crate::Resettable for FCFETCH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
