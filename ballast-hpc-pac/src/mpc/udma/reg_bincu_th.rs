#[doc = "Register `REG_BINCU_TH` reader"]
pub struct R(crate::R<REG_BINCU_TH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_BINCU_TH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_BINCU_TH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_BINCU_TH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_BINCU_TH` writer"]
pub struct W(crate::W<REG_BINCU_TH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_BINCU_TH_SPEC>;
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
impl From<crate::W<REG_BINCU_TH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_BINCU_TH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_BINCU_TH` reader - "]
pub struct REG_BINCU_TH_R(crate::FieldReader<u32>);
impl REG_BINCU_TH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_BINCU_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_BINCU_TH_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_BINCU_TH` writer - "]
pub struct REG_BINCU_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BINCU_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_bincu_th(&self) -> REG_BINCU_TH_R {
        REG_BINCU_TH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_bincu_th(&mut self) -> REG_BINCU_TH_W {
        REG_BINCU_TH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER binarization threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_bincu_th](index.html) module"]
pub struct REG_BINCU_TH_SPEC;
impl crate::RegisterSpec for REG_BINCU_TH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_bincu_th::R](R) reader structure"]
impl crate::Readable for REG_BINCU_TH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_bincu_th::W](W) writer structure"]
impl crate::Writable for REG_BINCU_TH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_BINCU_TH to value 0"]
impl crate::Resettable for REG_BINCU_TH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
