#[doc = "Register `Line_loop_en` reader"]
pub struct R(crate::R<LINE_LOOP_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE_LOOP_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE_LOOP_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE_LOOP_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Line_loop_en` writer"]
pub struct W(crate::W<LINE_LOOP_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE_LOOP_EN_SPEC>;
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
impl From<crate::W<LINE_LOOP_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE_LOOP_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Line_loop_en` reader - If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose."]
pub type LINE_LOOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `Line_loop_en` writer - If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose."]
pub type LINE_LOOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINE_LOOP_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose."]
    #[inline(always)]
    pub fn line_loop_en(&self) -> LINE_LOOP_EN_R {
        LINE_LOOP_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose."]
    #[inline(always)]
    #[must_use]
    pub fn line_loop_en(&mut self) -> LINE_LOOP_EN_W<0> {
        LINE_LOOP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line_loop_en](index.html) module"]
pub struct LINE_LOOP_EN_SPEC;
impl crate::RegisterSpec for LINE_LOOP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line_loop_en::R](R) reader structure"]
impl crate::Readable for LINE_LOOP_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line_loop_en::W](W) writer structure"]
impl crate::Writable for LINE_LOOP_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Line_loop_en to value 0"]
impl crate::Resettable for LINE_LOOP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
